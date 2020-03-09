//! Universal hash function implementation
use rand::Rng;

const PRIME: u64 = 0x7FFFFFFF;

/// How big the target key-space is
pub const SIZE: u64 = 128;

/// The universal hash function, can be evaluated
pub struct UniversalHashFunction {
    n: u64,
    m: u64,
}

impl UniversalHashFunction {
    /// Get a new random function from the family pool
    pub fn new() -> UniversalHashFunction {
        let n = rand::thread_rng().gen_range(0, PRIME);
        let m = rand::thread_rng().gen_range(1, PRIME);
        UniversalHashFunction { n, m }
    }

    /// Evaluate the function for a particular value,
    /// will return a value between 0 and SIZE
    pub fn evaluate(&self, x: u32) -> u32 {
        let x = x as u64;
        (((x * self.n + self.m) % PRIME) % SIZE) as u32
    }
}
