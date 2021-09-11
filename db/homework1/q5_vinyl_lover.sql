select distinct t1.name
from
    release t1
    inner join medium t2 on t1.id = t2.release
    inner join medium_format t3 on t3.id = t2.format
    inner join artist_credit t4 on t4.id = t1.artist_credit
    inner join artist_credit_name t5 on t4.id = t5.artist_credit
    inner join release_info t6 on t6.release = t1.id
where
    t5.name = 'Coldplay' and
    t3.name like '%Vinyl'
order by
    date_year,
    date_month,
    date_day;