use rand::Rng;

const PRIME: u32 = 0x7FFFFFFF;

struct UniversalHashFunction {
    n: u32,
    m: u32,
}

impl UniversalHashFunction {
    fn new() -> UniversalHashFunction {
        let n = rand::thread_rng().gen_range(0, PRIME);
        let m = rand::thread_rng().gen_range(1, PRIME);
        UniversalHashFunction { n, m }
    }

    fn evaluate(&self, x: u32) -> u32 {
        let t: u64 = x as u64 * self.n as u64;
        ((t + self.m as u64) % PRIME as u64) as u32 % 128
    }
}

fn main() {
    let a = 8274;
    let b = 918;
    let iterations = 100000;
    let mut collisions = 0;

    for _ in 0..iterations {
        let hash = UniversalHashFunction::new();
        let keys = (hash.evaluate(a), hash.evaluate(b));
        if keys.0 == keys.1 {
            collisions = collisions + 1;
        }
    }

    println!("Collisions: {} Expected: {}", collisions, iterations / 128);
}
