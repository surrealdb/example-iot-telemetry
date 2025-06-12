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
use surrealdb::{Surreal, engine::remote::ws::Client};
use tokio::{runtime::Runtime, sync::Mutex};

#[derive(Deserialize)]
pub struct SensorWindow {
    // id: RecordId,
    pub sensor: String,
    pub values: Vec<f64>,
}

pub type SensorData = HashMap<String, SensorWindow>;

pub fn queries_run(
    db: Arc<Mutex<Surreal<Client>>>,
    running: Arc<AtomicBool>,
    values: Arc<RwLock<SensorData>>,
    window_in_minutes: Arc<RwLock<u32>>,
) -> () {
    let rt = Runtime::new().unwrap();
    while running.load(Ordering::Relaxed) {
        let mut _values: Vec<SensorWindow> = vec![];
        rt.block_on(async {
            {
                let db = db.lock().await;
                let res = db
                    .query(format!(
                        r#"select
    record::id(sensor) as sensor,
    array::flatten(value) as values
from reading where id[0] > time::now() - {}m
group by sensor"#,
                        window_in_minutes.read().unwrap()
                    ))
                    .await;
                if let Ok(mut res) = res {
                    let res: surrealdb::Result<Vec<SensorWindow>> = res.take(0);
                    if let Ok(res) = res {
                        _values = res;
                    }
                }
            }
            {
                let mut values = values.write().unwrap();
                for win in _values {
                    values.insert(win.sensor.clone(), win);
                }
            }
        });
        thread::sleep(Duration::from_millis(100));
    }
}
