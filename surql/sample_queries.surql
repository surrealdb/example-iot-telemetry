SELECT
    id, sensor, value
FROM reading:[time::now() - 1h]..=[time::now()]
WHERE value > 0.8;

LET $temp = (SELECT VALUE value FROM reading:[time::now() - 10m]..=[time::now()]);
RETURN math::percentile($temp, 30) - 1.4 * math::interquartile($temp);
RETURN math::percentile($temp, 70) + 1.4 * math::interquartile($temp);
