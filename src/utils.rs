use std::collections::VecDeque;
use std::time::Duration;
use std::time::Instant;

pub fn keep_window(recent_values: &mut VecDeque<(Instant, f64)>, noise_value: f64) -> f64 {
    let now = Instant::now();
    let one_minute = Duration::from_secs(60);

    // add value to avg queue
    recent_values.push_back((now, noise_value));

    // clean old values
    while let Some((timestamp, _)) = recent_values.front() {
        if now.duration_since(*timestamp) > one_minute {
            recent_values.pop_front();
        } else {
            break; // The queue is ordered, so we can stop
        }
    }

    // calculate the average of the remaining values
    let last_min_avg = if !recent_values.is_empty() {
        recent_values.iter().map(|&(_, val)| val).sum::<f64>() / recent_values.len() as f64
    } else {
        0.0 // No data in the last minute
    };

    last_min_avg
}
