remove table last_minute_avgs;
define table last_minute_avgs as
    select
        sensor,
        math::mean(value) as avg,
        last_min_avg
    -- doesn't work:
    --   from reading:[time::now() - 30m]..=[time::now()]
    -- this does:
    from reading where id[0] > time::now() - 1m
    group by sensor;

select * from last_minute_avgs;
