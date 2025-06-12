REMOVE TABLE last_minute_avgs;
DEFINE TABLE last_minute_avgs AS
    SELECT
        sensor,
        math::mean(value) AS avg,
        last_min_avg
    -- doesn't work:
    --   from reading:[time::now() - 30m]..=[time::now()]
    -- this does:
    FROM reading WHERE id[0] > time::now() - 1m
    GROUP BY sensor;

SELECT * FROM last_minute_avgs;
