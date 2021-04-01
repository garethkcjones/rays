use rand::prelude::*;

/**
 * Returns a random `f64` in [0.0, 1.0).
 */
#[must_use]
pub fn f64() -> f64 {
    rand::thread_rng().gen()
}

/**
 * Returns a random `f64` in [min, max).
 */
#[must_use]
pub fn f64_in(min: f64, max: f64) -> f64 {
    min + (max - min) * f64()
}

/**
 * Returns a random `i32` in [min, max].
 */
#[must_use]
pub fn i32_in(min: i32, max: i32) -> i32 {
    f64_in(f64::from(min), f64::from(max) + 1.0) as _
}

/**
 * Returns a random `usize` in [min, max].
 */
#[must_use]
pub fn usize_in(min: usize, max: usize) -> usize {
    f64_in(min as f64, max as f64 + 1.0) as _
}
