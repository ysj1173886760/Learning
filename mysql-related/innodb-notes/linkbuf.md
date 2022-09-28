# Innodb-1 Linkbuf

代码主要在`innobase/include/ut0link_buf.h`中

link buf的作用在注释中有写到

>  Link buffer - concurrent data structure which allows:
>
> * concurrent addition of links
> * single-threaded tracking of connected path created by links
> * limited size of window with holes (missing links)

在redo log中的作用就是允许并发的写入log，但是flush线程只允许刷入连续的log。所以通过link buf来追踪这些log什么时候变得连续。

这里为了lock free所以选择了一种数组的实现方式。其实如果不考虑lock-free感觉更直观的方式是用`std::map`什么的来追踪这些range

三个成员变量：

```cpp
  /** Capacity of the buffer. */
  size_t m_capacity;

  /** Pointer to the ring buffer (unaligned). */
  std::atomic<Distance> *m_links;

  /** Tail pointer in the buffer (expressed in original unit). */
  alignas(ut::INNODB_CACHE_LINE_SIZE) std::atomic<Position> m_tail;
```

m_capacity是link buf的大小，长度是2的若干次幂。

m_links则是这个数组，全都是`atomic<uint64>`，因为是用来存储lsn的。是环状buffer。

m_tail则指向link buf的尾部，并会不断的递增。

link buf的核心就两个函数：`add_link`, `advance_tail`

```cpp
inline void Link_buf<Position>::add_link(Position from, Position to) {
  const auto index = slot_index(from);
  auto &slot = m_links[index];
  slot.store(to);
}
```

add_link这里就是获取到from对应到数组中的下标（mod一下数组长度），然后存上终点的位置即可。

`advance_tail`是个可以支持并发更新tail的函数。其实如果不支持并发更新而是单线程更新的话会简单很多，一个线程负责拿tail，然后跟着link往前推进即可。

```cpp
bool Link_buf<Position>::advance_tail_until(Stop_condition stop_condition,
                                            uint32_t max_retry) {
  /* multi threaded aware */
  auto position = m_tail.load(std::memory_order_acquire);
  auto from = position;
  uint32_t retry = 0;
  while (true) {
    auto index = slot_index(position);
    auto &slot = m_links[index];
    auto next_load = slot.load(std::memory_order_acquire);
    if (next_load >= position + m_capacity) {
      /* either we wrapped and tail was advanced meanwhile,
      or there is link start_lsn -> end_lsn of length >= m_capacity */
      position = m_tail.load(std::memory_order_acquire);
      if (position != from) {
        from = position;
        continue;
      }
    }
    if (next_load <= position || stop_condition(position, next_load)) {
      /* nothing to advance for now */
      return false;
    }
    /* try to lock as storing the end */
    if (slot.compare_exchange_strong(next_load, position,
                                     std::memory_order_acq_rel)) {
      /* it could happen, that after thread read position = m_tail.load(),
      it got scheduled out for longer; when it comes back it might still
      see the link going forward in that slot but m_tail could have been
      already advanced forward (as we do not reset slots when traversing
      them); thread needs to re-check if m_tail is still behind the slot. */
      position = m_tail.load(std::memory_order_acquire);
      if (position == from) {
        /* confirmed. can advance m_tail exclusively */
        position = next_load;
        break;
      }
    }
    retry++;
    if (retry > max_retry) {
      /* give up */
      return false;
    }
    UT_RELAX_CPU();
    position = m_tail.load(std::memory_order_acquire);
    if (position == from) {
      /* no progress? */
      return false;
    }
    from = position;
  }
  while (true) {
    Position next;
    bool stop = next_position(position, next);
    if (stop || stop_condition(position, next)) {
      break;
    }
    position = next;
  }
  /* unlock */
  m_tail.store(position, std::memory_order_release);
  if (position == from) {
    return false;
  }
  return true;
}
```

这里为了支持并发的tail做了一堆的处理。

具体来说，并发的tail也需要一个独占的权限来保证最终只有一个人更新。

他这里的做法就是去CAS这个tail，成功者会将tail处的link设为他自己，这样其他人就会认为tail的link还没有接上，就会推出advance tail的过程。相当于这个成功者把tail那一段的link给拿了过来。他会自己尝试向前更新，并最终更新tail

对应到代码中，第一个while的作用就是尝试获取这个锁，或者叫独占权。

第二个while的作用就是不断的通过`next_position`找下一个位置。

最后更新m_tail，把能够串联起来的最大位置添回去。

剩下一些helper function就没啥东西了