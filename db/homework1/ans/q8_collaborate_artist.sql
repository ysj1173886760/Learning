Select count(distinct artist)
From artist_credit_name
Where artist_credit in (
        select artist_credit
        from artist_credit_name
        where name = 'Ariana Grande'
    );