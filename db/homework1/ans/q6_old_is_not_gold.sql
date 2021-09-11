Select decade,
    count(*) as cnt
from (
        select (CAST((date_year / 10) as int) * 10) || 's' as decade
        from release
            inner join release_info on release.id = release_info.release
        where release.status = 1
            and date_year >= 1900
    )
Group by decade
Order by cnt desc,
    decade desc;