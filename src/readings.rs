use std::sync::Arc;

use noise::{NoiseFn, Perlin};
use rand::Rng;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reading {
    pub value: f64,
    pub last_min_avg: f64,
}

pub async fn store_reading(db: &Arc<Mutex<Surreal<Client>>>, sensor: &str, reading: Reading) -> () {
    let db = db.lock().await;
    let query = r#"
        CREATE reading:[
            time::now(),
        ]
        SET
            sensor = $sensor,
            value = $value,
            last_min_avg = $last_min_avg
    "#;
    let res = db
        .query(query)
        .bind(("sensor", sensor.to_string()))
        .bind(("value", reading.value))
        .bind(("last_min_avg", reading.last_min_avg))
        .await;
    match res {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    };
}

pub fn get_reading(perlin: &Perlin) -> f64 {
    // Generate a random 3D point for the Perlin noise input
    let x = rand::rng().random_range(-10.0..10.0);
    let y = rand::rng().random_range(-10.0..10.0);
    let z = rand::rng().random_range(-10.0..10.0);

    let noise_value = perlin.get([x, y, z]);
    noise_value
}
