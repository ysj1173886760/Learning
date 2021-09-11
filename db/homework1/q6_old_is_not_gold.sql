select
    (CAST((date_year / 10) as int) * 10) || 's' as decade,
    count(*) as cnt
from
    release t1
    inner join release_info t2 on t1.id = t2.release
where
    t1.status = 1 and
    t2.date_year >= 1900
group by decade
order by cnt desc;