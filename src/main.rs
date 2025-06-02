use clap::Parser;
use noise::Perlin;
use rand::Rng;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Instant;
use std::{thread, time::Duration};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

use readings::store_reading;

use crate::readings::{Reading, get_reading};
use crate::utils::keep_window;

mod readings;
mod utils;

/// A simple Rust program that prints random Perlin noise numbers in concurrent threads.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of concurrent threads
    #[clap(short, long, default_value = "4")]
    threads: usize,

    /// Delay between each print in milliseconds
    #[clap(short, long, default_value = "1000")]
    delay: u64,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let args = Args::parse();

    let mut handles = vec![];

    let db = Surreal::new::<Ws>("0.0.0.0:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("telemetry-simulator").use_db("demo").await?;
    let db_ref: Arc<Mutex<Surreal<Client>>> = Arc::new(Mutex::new(db));

    println!(
        "Starting {} threads to generate Perlin noise...",
        args.threads
    );

    for i in 0..args.threads {
        let rt = Runtime::new().unwrap();
        let delay = args.delay;
        let db_clone = Arc::clone(&db_ref);
        let handle = thread::spawn(move || {
            let sensor = format!("Sensor {}", i);
            let perlin = Perlin::default(); // Initialize Perlin with a random seed
            // store (timestamp, value) pairs
            let mut recent_values: VecDeque<(Instant, f64)> = VecDeque::new();

            // delay start randomly
            thread::sleep(Duration::from_millis(
                (delay as f64 * rand::rng().random_range(0.0..1.)) as u64,
            ));
            println!("Thread {} started.", i);

            loop {
                let noise_value = get_reading(&perlin);
                let last_min_avg = keep_window(&mut recent_values, noise_value);

                println!("Thread {}: Perlin Noise value = {:.4}", i, noise_value);
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

    Ok(())
}
