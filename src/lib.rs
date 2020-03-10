//! Universal hash function implementation
use rand::Rng;

const PRIME: u32 = 0x1FFF;

/// How big the target key-space is
pub const SIZE: u32 = 128;

/// The universal hash function, can be evaluated
pub struct UniversalHashFunction {
    n: u32,
    m: u32,
}

impl UniversalHashFunction {
    /// Get a new random function from the family pool
    pub fn new() -> UniversalHashFunction {
        let n = rand::thread_rng().gen_range(0, PRIME);
        let m = rand::thread_rng().gen_range(1, PRIME);
        UniversalHashFunction { n, m }
    }

    /// Evaluate the function for a particular string,
    /// will return a value between 0 and SIZE
    pub fn evaluate(&self, x: &str) -> u32 {
        let mut hash = 0;
        for c in x.bytes() {
            hash = hash ^ (self.m * (c as u32) + self.n)
        }
        hash % PRIME % SIZE
    }
}
