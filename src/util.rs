use std::time::{Duration, Instant};

#[inline]
pub fn ns_per_frame(fps: usize) -> u64 {
    (Duration::from_secs(1).as_nanos() as f64 / fps as f64).round() as u64
}

#[allow(dead_code)]
pub fn sleep_for_constant_rate(rate: usize, start: Instant) {
    let ns_per_frame = ns_per_frame(rate);
    let duration = Duration::from_nanos(ns_per_frame);
    let elapsed = start.elapsed();
    if elapsed < duration {
        std::thread::sleep(duration - elapsed);
    }
}
