use std::{
    collections::HashMap,
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use serde::Deserialize;
use surrealdb::{RecordId, Surreal, engine::remote::ws::Client};
use tokio::{runtime::Runtime, sync::Mutex};

#[derive(Deserialize)]
pub struct SensorWindow {
    // id: RecordId,
    pub sensor: String,
    pub values: Vec<f64>,
}

#[derive(Deserialize)]
pub struct SensorAverage {
    // id: RecordId,
    pub sensor: RecordId,
    pub avg: f64,
}

pub type SensorData = HashMap<String, SensorWindow>;

async fn query_averages(db: &Arc<Mutex<Surreal<Client>>>) -> surrealdb::Result<Vec<SensorAverage>> {
    let q = r#"
        select
            sensor,
            math::mean(value) as avg,
            last_min_avg
        from reading where id[0] > time::now() - 1m
        group by sensor
        "#;
    let db = db.lock().await;
    let mut res = db.query(q).await?;
    Ok(res.take::<Vec<SensorAverage>>(0).unwrap())
}

async fn query_data(
    db: &Arc<Mutex<Surreal<Client>>>,
    window_in_minutes: &Arc<RwLock<u32>>,
) -> surrealdb::Result<Vec<SensorWindow>> {
    let q = format!(
        r#"
        select
            record::id(sensor) as sensor,
            array::flatten(value) as values
        from reading where id[0] > time::now() - {}m
        group by sensor
        "#,
        window_in_minutes.read().unwrap()
    );
    let db = db.lock().await;
    let mut res = db.query(q).await?;
    res.take::<Vec<SensorWindow>>(0)
}

pub fn queries_run(
    db: Arc<Mutex<Surreal<Client>>>,
    running: Arc<AtomicBool>,
    values: Arc<RwLock<SensorData>>,
    avgs: Arc<RwLock<Vec<SensorAverage>>>,
    window_in_minutes: Arc<RwLock<u32>>,
    delay_in_ms: u64,
) -> () {
    let rt = Runtime::new().unwrap();
    while running.load(Ordering::Relaxed) {
        let mut _values: Vec<SensorWindow> = vec![];
        rt.block_on(async {
            // - Query data
            if let Ok(_values) = query_data(&db, &window_in_minutes).await {
                let mut values = values.write().unwrap();
                for win in _values {
                    values.insert(win.sensor.clone(), win);
                }
            }
            // - Query averages from pre-computed table
            if let Ok(_avgs) = query_averages(&db).await {
                let mut avgs = avgs.write().unwrap();
                *avgs = _avgs
            }
        });
        thread::sleep(Duration::from_millis(delay_in_ms));
    }
}
