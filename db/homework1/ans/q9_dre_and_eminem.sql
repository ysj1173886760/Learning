with duos_list (id1, id2, count) as (
    select a1.artist as id1,
        a2.artist as id2,
        count(*) as c
    from artist_credit_name a1
        inner join artist_credit_name a2 on a1.artist_credit = a2.artist_credit
        inner join release r on a2.artist_credit = r.artist_credit
        inner join artist a3 on a1.artist = a3.id
        inner join artist a4 on a2.artist = a4.id
        inner join artist_type a5 on a3.type = a5.id
        inner join artist_type a6 on a4.type = a6.id
        inner join language l on r.language = l.id
    where a3.name < a4.name
        and a5.name = "Person"
        and a6.name = "Person"
        and l.name = 'English'
        and a3.begin_date_year > 1960
        and a4.begin_date_year > 1960
    group by a1.artist,
        a2.artist
)
select *
from (
        select row_number () over (
                order by count desc,
                    a1.name,
                    a2.name
            ) as rank,
            a1.name as name1,
            a2.name as name2,
            count
        from duos_list d
            inner join artist a1 on d.id1 = a1.id
            inner join artist a2 on d.id2 = a2.id
    )
where name1 = 'Dr. Dre'
    and name2 = 'Eminem';