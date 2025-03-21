use rayon::prelude::*;
use rand_xoshiro::{rand_core::{SeedableRng, RngCore}, Xoshiro256PlusPlus};
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    let total_points: u64 = 1_000_000_000; // 1 billion points
    let num_threads = 24;

    // Use an atomic counter for hits
    let hits = AtomicU64::new(0);

    // Use Rayon's parallel iterator to process points in parallel
    (0..num_threads).into_par_iter().for_each(|_| {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42); // Seed the PRNG
        let mut local_hits: u64 = 0;

        for _ in 0..(total_points / num_threads as u64) {
            let x = (rng.next_u64() as f64) / (u64::MAX as f64); // Generate random f64
            let y = (rng.next_u64() as f64) / (u64::MAX as f64); // Generate another random f64

            if x * x + y * y <= 1.0 {
                local_hits += 1;
            }
        }

        // Safely update the shared hits counter using atomic operations
        hits.fetch_add(local_hits, Ordering::Relaxed);
    });

    let hits = hits.into_inner();
    let pi = 4.0 * (hits as f64) / (total_points as f64);

    println!("Estimated value of π: {:.100}", pi); // Print with 100 decimal places
}
