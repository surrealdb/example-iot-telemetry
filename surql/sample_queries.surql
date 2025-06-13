SELECT
    id, sensor, value
FROM reading:[time::now() - 1h]..=[time::now()]
WHERE value > 0.8;

LET $temp = (SELECT VALUE value FROM reading:[time::now() - 10m]..=[time::now()]);
RETURN math::percentile($temp, 30) - 1.4 * math::interquartile($temp);
RETURN math::percentile($temp, 70) + 1.4 * math::interquartile($temp);


select
    sensor,
    math::mean(value) as avg,
    last_min_avg
-- doesn't work:
--   from reading:[time::now() - 30m]..=[time::now()]
-- this does:
from reading where id[0] > time::now() - 1m
group by sensor;
