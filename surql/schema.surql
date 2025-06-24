USE NS `telemetry-simulator` DB demo;

-- Alert
DEFINE TABLE alert SCHEMAFULL;
DEFINE FIELD outlier ON TABLE alert TYPE record<reading>;
DEFINE FIELD message ON TABLE alert TYPE string;

-- Reading
DEFINE TABLE reading SCHEMAFULL;
DEFINE FIELD sensor ON TABLE reading TYPE record<sensor>;
DEFINE FIELD value ON TABLE reading TYPE float;
DEFINE FIELD last_min_avg ON TABLE reading TYPE float;

-- Sensor
DEFINE TABLE sensor SCHEMAFULL;

-- Site
DEFINE TABLE site SCHEMAFULL;

-- Relations
DEFINE TABLE created_alert TYPE RELATION IN sensor OUT alert;
DEFINE TABLE located_at TYPE RELATION IN sensor OUT site;
