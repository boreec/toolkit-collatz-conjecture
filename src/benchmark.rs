use std::time::Instant;

use crate::core::CollatzSequence;

pub fn benchmark(n: u128) {
    let compute_start = Instant::now();
    compute_sequences(n);
    let compute_duration = compute_start.elapsed();
    println!(
        "Time elapsed to compute steps on sequences from 1 to {}: {:?}",
        n, compute_duration
    )
}

pub fn compute_sequences(n: u128) {
    for i in 0..n {
        let _ = CollatzSequence::new(i);
    }
}
