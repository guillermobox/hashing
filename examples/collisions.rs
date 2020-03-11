fn main() {
    let (a, b, iterations) = ("potato", "tomato", 100000);
    let mut collisions = 0;

    for _ in 0..iterations {
        let hash = hashing::UniversalHashFunction::<u32>::new();
        if hash.evaluate(a) == hash.evaluate(b) {
            collisions = collisions + 1;
        }
    }

    println!(
        "Collisions: {} Expected: {}",
        collisions,
        iterations / hashing::SIZE
    );
}
