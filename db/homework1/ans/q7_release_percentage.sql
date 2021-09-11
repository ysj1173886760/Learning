with past_year_release (year, month) as (
    select date_year,
        date_month
    from release_info r1
        inner join release r2 on r1.release = r2.id
    where (
            (
                date_year = 2019
                and date_month >= 7
            )
            or (
                date_year = 2020
                and date_month <= 7
            )
        )
)
select cast(year as varchar) || '.' || (
        case
            when month < 10 then '0'
            else ''
        end
    ) || cast(month as varchar) as date,
    round(
        count(*) * 100.0 / (
            select count(*)
            from past_year_release
        ),
        2
    )
from past_year_release
group by date
order by date;