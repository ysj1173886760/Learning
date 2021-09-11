select distinct r1.name as rname
from artist_credit_name a1
    inner join artist_credit a2 on a1.artist_credit = a2.id
    inner join release r1 on a2.id = r1.artist_credit
    inner join release_info r2 on r1.id = r2.release
    inner join medium m1 on r1.id = m1.release
    inner join medium_format m2 on m1.format = m2.id
where a1.name = 'Coldplay'
    and m2.name like '%Vinyl'
order by date_year,
    date_month,
    date_day;