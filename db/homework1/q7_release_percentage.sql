with tmp (year, month, cnt) as (
    select t2.date_year, t2.date_month, count(*)
    from
        release t1
        inner join release_info t2 on t1.id = t2.release
    where
        (t2.date_year = 2019 and t2.date_month >= 7) or
        (t2.date_year = 2020 and t2.date_month <= 7)
    group by t2.date_year, t2.date_month
)
select cast(year as varchar) || '.' || (
        case
            when month < 10 then '0'
            else ''
        end
    ) || cast(month as varchar) as date,
    round(cnt * 100.0 / (select sum(cnt) from tmp), 2)
from tmp
order by date;