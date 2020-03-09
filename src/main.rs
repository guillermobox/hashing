use rand::Rng;

const PRIME: u64 = 0x7FFFFFFF;
const SIZE: u64 = 128;

struct UniversalHashFunction {
    n: u64,
    m: u64,
}

impl UniversalHashFunction {
    fn new() -> UniversalHashFunction {
        let n = rand::thread_rng().gen_range(0, PRIME);
        let m = rand::thread_rng().gen_range(1, PRIME);
        UniversalHashFunction { n, m }
    }

    fn evaluate(&self, x: u32) -> u32 {
        let x = x as u64;
        (((x * self.n + self.m) % PRIME) % SIZE) as u32
    }
}

fn main() {
    let a = 8274;
    let b = 918;
    let iterations = 100000;
    let mut collisions = 0;

    for _ in 0..iterations {
        let hash = UniversalHashFunction::new();
        if hash.evaluate(a) == hash.evaluate(b) {
            collisions = collisions + 1;
        }
    }

    println!("Collisions: {} Expected: {}", collisions, iterations / 128);
}
