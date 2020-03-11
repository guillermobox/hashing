//! Universal hash function implementation
use rand::Rng;

const PRIMEU32: u32 = 0x1FFF;
const PRIMEU64: u64 = 0x7FFFFFFF;

/// How big the target key-space is
pub const SIZE: u32 = 128;

/// The universal hash function,
/// that can be assembled for 32bits, 64bits, or other types
pub struct UniversalHashFunction<T> {
    n: T,
    m: T,
}

impl UniversalHashFunction<u32> {
    pub fn new() -> UniversalHashFunction<u32> {
        let n = rand::thread_rng().gen_range(0, PRIMEU32);
        let m = rand::thread_rng().gen_range(1, PRIMEU32);
        UniversalHashFunction { n, m }
    }

    pub fn evaluate(&self, x: &str) -> u32 {
        let mut hash = 0;
        for c in x.bytes() {
            hash = hash ^ (self.m * (c as u32) + self.n)
        }
        hash % PRIMEU32 % SIZE
    }
}

impl UniversalHashFunction<u64> {
    pub fn new() -> UniversalHashFunction<u64> {
        let n = rand::thread_rng().gen_range(0, PRIMEU64);
        let m = rand::thread_rng().gen_range(1, PRIMEU64);
        UniversalHashFunction { n, m }
    }

    pub fn evaluate(&self, x: u32) -> u32 {
        let x = x as u64;
        (((x * self.n + self.m) % PRIMEU64) % SIZE as u64) as u32
    }
}
