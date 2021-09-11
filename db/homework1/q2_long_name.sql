select work.name, work_type.name
from
	work
	inner join(
		select max(length(work.name)) as max_length, work.type as type
		from work
		group by work.type
	) as newtable 
		on newtable.max_length = length(work.name) and
			newtable.type = work.type
	inner join work_type on work_type.id = work.type
order by work.type asc, work.name asc;
