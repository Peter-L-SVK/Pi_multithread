use rayon::prelude::*;
use rand::Rng;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    let total_points: u64 = 1_000_000_000; // 1 billion points
    let num_threads = 24;

    // Use thread-local counters
    let hits: Vec<AtomicU64> = (0..num_threads).map(|_| AtomicU64::new(0)).collect();

    // Use Rayon's parallel iterator to process points in parallel
    (0..num_threads).into_par_iter().for_each(|i| {
        let mut rng = rand::rng();
        let mut local_hits: u64 = 0;

        for _ in 0..(total_points / num_threads as u64) {
            let x: f64 = rng.random();
            let y: f64 = rng.random();

            if x * x + y * y <= 1.0 {
                local_hits += 1;
            }
        }

        // Update the thread-local counter
        hits[i].store(local_hits, Ordering::Relaxed);
    });

    // Combine the thread-local counters
    let total_hits: u64 = hits.iter().map(|h| h.load(Ordering::Relaxed)).sum();
    let pi = 4.0 * (total_hits as f64) / (total_points as f64);

    println!("Estimated value of π: {:.100}", pi); // Print with 100 decimal places
}
