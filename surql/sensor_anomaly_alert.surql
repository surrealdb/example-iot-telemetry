REMOVE EVENT sensor_anomaly_alert ON reading;

-- Trigger the event on when a record is created
DEFINE EVENT sensor_anomaly_alert ON reading WHEN $event = 'CREATE'
THEN {
    LET $temp_past_hour = SELECT VALUE value FROM reading:[time::now() - 1h]..=[time::now()];
    -- Calculate both the upper and lower threshold for an outlier
    -- using the standard Q1 - 1.5 * IQR formula for the low outliers
    LET $low_outliers = (
        RETURN math::percentile($temp_past_hour, 25) - 1.5 * math::interquartile($temp_past_hour)
    );
    -- Q3 + 1.5 * IQR formula for the high outliers
    LET $high_outliers = (
        RETURN math::percentile($temp_past_hour, 75) + 1.5 * math::interquartile($temp_past_hour)
    );

    DEFINE PARAM OVERWRITE $message VALUE None;

    -- tip: you can also call http::post
    IF $after.value < $low_outliers {
        DEFINE PARAM OVERWRITE $message VALUE 'Outlier Detected: low'
    };

    IF $after.value > $high_outliers {
        DEFINE PARAM OVERWRITE $message VALUE 'Outlier Detected: high'
    };

    IF $message {
        LET $alert = CREATE alert CONTENT {
            outlier: $after,
            message: $message
        } RETURN id;

        RELATE ($after.sensor)->created_alert->$alert;
    }
};
