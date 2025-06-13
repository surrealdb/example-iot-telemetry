---
title: Telemetry Demo
sub_title: SurrealDB, time series, event triggers, graph
author: Martin Schaer <martin.schaer@surrealdb.com>
theme:
  name: surreal
---

Event trigger
===

```file +line_numbers
path: surql/sensor_anomaly_alert.surql
language: sql
```

<!-- end_slide -->

Live Alerts
===

```sql
live select * from alert;
```

<!-- end_slide -->

Graph queries
===

```sql
select *,->created_alert->alert from sensor;
```

<!-- end_slide -->
