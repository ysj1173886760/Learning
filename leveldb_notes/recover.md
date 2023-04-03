# LevelDB Recover

过一遍LevelDB的Recover流程。

1. DB::Open
2. DBImpl::Recover
   1. 给当前的db上锁。防止有什么并发的OpenDB之类的。
   2. 看看有没有CurrentFile，没有的话打开一个新的DB。
3. VersionSet::Recover
   1. 读取Current，找到Manifest文件。
   2. 读取Manifest，重放VersionEdit。
   3. 如果Manifest过大，或者是重新将Manifest文件通过Append的方式打开的时候失败了，就会放弃Reuse Manifest。
4. 按照file number顺序找到所有number大于等于Version->LogNumber的日志。重放Redo日志
   1. DMImpl::RecoverLogFile。按从小到大的顺序回放。当memtable过大的时候，就会原地写MemTable，并且会将save_manifest设为true，表示不再reuse manifest。
   2. 如果没有进行compaction，会尝试复用最后一个log file。
   3. 如果最后仍然存在mem table，会再进行一次Compaction。
   4. 更新Version中的Sequence为遇到的最大Sequence。
5. Recover结束，如果没剩下mem table，则会创建一个memtable，以及对应的WAL。
6. 在前面Recover VersionSet的时候，如果Reuse Manifest的话，会创建一个新的DescriptorLog，就是用来写VersionEdit的Writer。
   1. 否则的话，在第一次Apply VersionEdit的时候，会发现没有DescriptorLog，就会进行一次全量的Checkpoint。
   2. 然后在写入本次VersionEdit的时候，如果发现进行了全量Checkpoint，就会更新一下CurrentFile为当前Manifest的名字。
7. 最后尝试触发Compaction，就可以放锁开始正常的写入流程了。