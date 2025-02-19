use rayon::prelude::*;
use rand::Rng;
use std::sync::Mutex;

fn main() {
    let total_points: u64 = 1_000_000_000; // 1 billion points
    let num_threads = 24;

    // Divide the work among threads
    let hits = Mutex::new(0u64);

    println!("Calculation of Pi places:");
    // Use Rayon's parallel iterator to process points in parallel
    (0..num_threads).into_par_iter().for_each(|_| {
        let mut rng = rand::thread_rng();
        let mut local_hits: u64 = 0;

        for _ in 0..(total_points / num_threads as u64) {
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();

            if x * x + y * y <= 1.0 {
                local_hits += 1;
            }
        }

        // Safely update the shared hits counter
        let mut hits = hits.lock().unwrap();
        *hits += local_hits;
    });

    let hits = hits.into_inner().unwrap();
    let pi = 4.0 * (hits as f64) / (total_points as f64);

    println!("Estimated value of π: {:.100}", pi); // Print with 15 decimal places
}
