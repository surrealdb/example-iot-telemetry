---
title: Telemetry Demo
sub_title: SurrealDB, time series, events, table views
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

Sample queries
===

```file +line_numbers
path: surql/sample_queries.surql
language: sql
```

<!-- end_slide -->

Live view and pre-computed tables
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
