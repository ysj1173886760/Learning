Innodb写路径调用栈

```cpp
ha_innobase::write_row
  row_insert_for_mysql
  	如果table是系统表
      row_insert_for_mysql_using_cursor
    否则
  		row_insert_for_mysql_using_ins_graph
  			row_mysql_convert_row_to_innobase	// 将tuple从mysql format转化到innodb format
  			row_ins_step
  				row_ins
  					// step函数有点类似执行一个计划的感觉
  					row_ins_alloc_row_id_step
  					row_ins_get_row_from_query_block // 从select中构建row
  					row_ins_get_row_from_values // 从value list中构建row
  					// 然后遍历node中的每个index
  					row_ins_index_entry_step
  						row_ins_index_entry_set_vals	// 根据index构建对应的row
  						row_ins_index_entry
  							如果index是聚簇索引
  								row_ins_clust_index_entry
  									row_ins_clust_index_entry_low	// 这里区分optimisitc和pessimistic的区别是BTR_MODIFY_LEAF和BTR_MODIFY_TREE
  										pcur.open
  											btr_cur_search_to_nth_level
  										cursorr = pcur.get_btr_cur
  										btr_cur_optimistic_insert
  											btr_cur_ins_lock_and_undo
  											page_cur_tuple_insert
  												rec_convert_dtuple_to_rec
  												page_cur_insert_rec_low
  													page_cur_insert_rec_write_log
  										btr_cur_pessimistic_insert
  							如果是multi value entry	// 还不清楚这是什么类型的索引
  								row_ins_sec_index_multi_value_entry
  							否则
  								row_ins_sec_index_entry
```



