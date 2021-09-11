Select artist.name,
    count(distinct artist_alias.name) as num
From artist
    inner join artist_alias on artist.id = artist_alias.artist
Where artist.begin_date_year > 1950
    and area = 221
Group by artist.id
Order by num desc
Limit 10;