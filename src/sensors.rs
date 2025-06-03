use noise::Perlin;
use rand::Rng;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use std::{thread, time::Duration};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

use crate::readings::{Reading, get_reading, store_reading};
use crate::utils::keep_window;

pub fn sensors_run(
    threads: usize,
    delay: u64,
    running: Arc<AtomicBool>,
    db: Arc<Mutex<Surreal<Client>>>,
) -> () {
    let mut handles = vec![];
    for i in 0..threads {
        let rt = Runtime::new().unwrap();
        let db_clone = Arc::clone(&db);
        let running_clone = running.clone();
        let handle = thread::spawn(move || {
            let sensor = format!("Sensor {}", i);
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
                        &sensor,
                        Reading {
                            value: noise_value,
                            last_min_avg,
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
}
