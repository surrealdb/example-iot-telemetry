---
title: Telemetry Demo
sub_title: SurrealDB, time series, event triggers, graph
author: Martin Schaer <martin.schaer@surrealdb.com>
theme:
  name: surreal
---

Solution
===

![image:w:100%](docs/solution.png)

<!-- end_slide -->

DB schema
===

![image:w:100%](docs/db-diagram.png)

<!-- end_slide -->

Event trigger
===

```file +line_numbers
path: surql/sensor_anomaly_alert.surql
language: sql
```

<!-- end_slide -->

Live Alerts
===
<!-- column_layout: [1, 1] -->
<!-- column: 0 -->

```sql
live select * from alert;
```
<!-- column: 1 -->
![image:w:100%](docs/live-query.png)

<!-- end_slide -->

Graph queries
===


```file +line_numbers
path: surql/graph.surql
language: sql
```

<!-- column_layout: [1, 1] -->
<!-- column: 0 -->

## Alerts per site
```yaml
[
	{
		alerts: [
			{
				message: 'High',
				outlier: reading:[
					d'2025-06-24T14:47:30.329Z'
				]
			},
			...
		],
		id: site:⟨site-1⟩
	},
	{
		alerts: [
			{
				message: 'High',
				outlier: reading:[
					d'2025-06-24T15:31:42.449Z'
				]
			},
			...
		],
		id: site:⟨site-2⟩
	}
]
```

<!-- column: 1 -->
## Alerts per sensor
![image:w:100%](docs/graph.png)

<!-- end_slide -->
