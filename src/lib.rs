//! Universal hash function implementation
use rand::Rng;

const PRIMEU32: u32 = 0x1FFF;
const PRIMEU64: u64 = 0x7FFFFFFF;

pub struct IntegerHashFunction {
    n: u64,
    m: u64,
    size: u32,
}

impl IntegerHashFunction {
    pub fn new(size: u32) -> IntegerHashFunction {
        let n = rand::thread_rng().gen_range(0, PRIMEU64);
        let m = rand::thread_rng().gen_range(1, PRIMEU64);
        IntegerHashFunction { n, m, size }
    }
    pub fn hash(&self, x: u32) -> u32 {
        let x = x as u64;
        (((x * self.n + self.m) % PRIMEU64) % self.size as u64) as u32
    }
}

pub struct StringHashFunction {
    n: u32,
    m: u32,
    size: u32,
}

impl StringHashFunction {
    pub fn new(size: u32) -> StringHashFunction {
        let n = rand::thread_rng().gen_range(0, PRIMEU32);
        let m = rand::thread_rng().gen_range(1, PRIMEU32);
        StringHashFunction { n, m, size }
    }
    pub fn hash(&self, x: &str) -> u32 {
        let mut hash = 0;
        for c in x.bytes() {
            hash = hash ^ (self.m * (c as u32) + self.n)
        }
        hash % PRIMEU32 % self.size
    }
}

