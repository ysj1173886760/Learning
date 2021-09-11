with tmp as (
select
    row_number() over (order by t1.id asc) as num,
    t1.name as name
from
    artist_alias t1
    inner join artist t2 on t1.artist = t2.id
where t2.name = 'The Beatles'
),
recur(num, name) as (
    select num, name
    from tmp
    where num = 1
    union all
    select tmp.num, r.name || ', ' || tmp.name
    from 
        tmp,
        recur as r
    where tmp.num = r.num + 1
)
select name
from recur
order by num desc
limit 1;