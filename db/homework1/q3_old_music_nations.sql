select area.name, count(*) as cnt
from 
    artist
    inner join area on artist.area = area.id
where begin_date_year < 1850
group by area.name
order by cnt desc
limit 10;