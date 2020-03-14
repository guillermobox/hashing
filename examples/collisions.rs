use hashing;

fn main() {
    let (a, b, iterations) = ("potato", "tomato", 100000);
    let mut collisions = 0;

    for _ in 0..iterations {
        let f = hashing::StringHashFunction::new(128);
        if f.hash(a) == f.hash(b) {
            collisions = collisions + 1;
        }
    }

    println!(
        "Collisions: {} Expected: {}",
        collisions,
        iterations / 128
    );
}
