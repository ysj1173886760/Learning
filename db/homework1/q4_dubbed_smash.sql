select t2.name, count(distinct t1.name) as cnt
from
    artist_alias t1
    inner join artist t2 on t1.artist = t2.id
    inner join area t3 on t2.area = t3.id
where t3.name = 'United Kingdom' and t2.begin_date_year > 1950
group by t2.id
order by cnt desc
limit 10;