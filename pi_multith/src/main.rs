use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;

fn main() {
    let total_points: u64 = 1_000_000_000; 
    let num_threads = 24;
    let points_per_thread = total_points / num_threads as u64;

    let hits = Arc::new(Mutex::new(0u64)); // Use u64 for hits

    let mut handles = vec![];

    println!("Calculation of Pi places:");
    for _ in 0..num_threads {
        let hits = Arc::clone(&hits);
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut local_hits: u64 = 0;

            for _ in 0..points_per_thread {
                let x: f64 = rng.gen();
                let y: f64 = rng.gen();

                if x * x + y * y <= 1.0 {
                    local_hits += 1;
                }
            }

            let mut hits = hits.lock().unwrap();
            *hits += local_hits;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let hits = *hits.lock().unwrap();
    let pi = 4.0 * (hits as f64) / (total_points as f64);
    println!("Estimated value of π: {:.100}", pi);
}

