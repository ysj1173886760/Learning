with c as (
      select row_number() over (
                  order by c.id asc
            ) as seqnum,
            c.name as name
      from artist_alias c
            join artist on c.artist = artist.id
      where artist.name = 'The Beatles'
),
flattened as (
      select seqnum,
            name as name
      from c
      where seqnum = 1
      union all
      select c.seqnum,
            f.name || ', ' || c.name
      from c
            join flattened f on c.seqnum = f.seqnum + 1
)
select name
from flattened
order by seqnum desc
limit 1;