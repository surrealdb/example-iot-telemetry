use noise::Perlin;
use rand::Rng;
use serde::Deserialize;
use std::collections::VecDeque;
use std::io::ErrorKind;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use std::{thread, time::Duration};
use surrealdb::engine::remote::ws::Client;
use surrealdb::{RecordId, Surreal};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

use crate::readings::{Reading, get_reading, store_reading};
use crate::utils::keep_window;

#[derive(Deserialize)]
struct Sensor {
    id: RecordId,
}

async fn insert_sensor(
    db: &Arc<Mutex<Surreal<Client>>>,
    sensor: &str,
) -> Result<Sensor, ErrorKind> {
    let db = db.lock().await;
    let s: Option<Sensor> = db
        .create(("sensor", sensor))
        .await
        .map_err(|_e| ErrorKind::Other)?;
    s.ok_or(ErrorKind::Other)
}

pub fn sensors_run(
    sensor_count: usize,
    delay: u64,
    running: Arc<AtomicBool>,
    db: Arc<Mutex<Surreal<Client>>>,
) -> Result<(), ErrorKind> {
    let mut handles = vec![];

    for i in 0..sensor_count {
        let rt = Runtime::new().unwrap();

        // add sensor to DB
        let sensor_name = format!("sensor-{}", i);
        let sensor_id = rt.block_on(async {
            match insert_sensor(&db, &sensor_name).await {
                Ok(s) => s.id,
                Err(_) => RecordId::from_table_key("sensor", sensor_name),
            }
        });

        let db_clone = Arc::clone(&db);
        let running_clone = running.clone();
        let handle = thread::spawn(move || {
            let perlin = Perlin::default(); // Initialize Perlin with a random seed
            // store (timestamp, value) pairs
            let mut recent_values: VecDeque<(Instant, f64)> = VecDeque::new();

            // delay start randomly
            thread::sleep(Duration::from_millis(
                (delay as f64 * rand::rng().random_range(0.0..1.)) as u64,
            ));

            while running_clone.load(Ordering::Relaxed) {
                let noise_value = get_reading(&perlin);
                let last_min_avg = keep_window(&mut recent_values, noise_value);

                rt.block_on(async {
                    store_reading(
                        &db_clone,
                        Reading {
                            value: noise_value,
                            last_min_avg,
                            sensor: sensor_id.clone(),
                        },
                    )
                    .await;
                });
                thread::sleep(Duration::from_millis(delay));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}
