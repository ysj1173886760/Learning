select a2.name,
    count(*) as c
from artist a1
    inner join area a2 on a1.area = a2.id
where begin_date_year < 1850
group by a1.area
order by c desc
limit 10;