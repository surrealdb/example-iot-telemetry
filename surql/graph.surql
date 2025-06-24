-- Alerts per sensor
SELECT *, ->created_alert->alert FROM sensor;

-- Sensor locations
SELECT *, ->located_at->site FROM sensor;

-- Alerts per site
SELECT
    id,
    <-located_at<-sensor->created_alert->alert.{message, outlier} AS alerts
FROM site
FETCH alerts;
