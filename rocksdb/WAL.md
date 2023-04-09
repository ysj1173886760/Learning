# WAL

简单看一下RocksDB的WAL

## SyncWAL

接口处有一个叫SyncWAL的函数，作用是把当前的日志同步到盘上。

相关的数据结构有一个`deque<LogWriterNumber>`，以及一个锁叫做`log_write_mutex_`。

其中`LogWriterNumber`中保存了log filter number，log writer，一个标识当前日志正在进行Sync的标记，以及pre_sync_size，代表在调用Sync之前的已经写入当前日志的数据大小，这里注释是说这个值会被记录在Manifest中，也就是说RocksDB会记录每个日志都有哪些数据被刷到了盘中（具体目的还不清楚）。

 SyncWAL中会取出当前MemTable的LogFileNumber，如果当前有日志正在进行Sync，判断的标记就是LogWriterNumber中的getting_sync，就会睡在一个CV上等待唤醒。

然后会把所有小于刚才记录的log file number的日志都调用PrepareForSync，表示当前日志正在进行Sync，并记录每个日志已经写入的数据量，即pre_sync_size。

然后把所有需要同步的日志都调用`SyncWithoutFlush`，表示不会写入数据（写入PageCache），只会调用Sync。这里调WithoutFlush的含义我也不是很懂，感觉一把刷下去就好了。。可能是有什么feature上的兼容吧。

刷完以后可能会同步一下原数据，这里调的是FsyncWithDirOptions，猜测就是创建新的文件后就需要Sync一下Dir。

结束后会调MarkLogsSynced，里面会判断下，如果一个日志已经全部都Sync了，就会把他从刚才的deque中移除，否则的话就会调用FinishSync，标记Sync已经结束，等待下一轮的Sync。

如果打开了`track_and_verify_wals_in_manifest`就会把刚才的变更记录到Manifest中，注释中说到他的用处就是用来及时发现Corruption。

如果要记录已经Sync的WAL的话，会调用`ApplyWALToManifest`来把这个VersionEdit记录下来。



