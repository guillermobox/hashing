use hashing::Hashable;

fn main() {
    let (a, b, iterations) = ("potato", "tomato", 100000);
    let mut collisions = 0;

    for _ in 0..iterations {
        let f = hashing::UniversalHashFunction::<u32>::new();
        if a.hash_with(&f) == b.hash_with(&f) {
            collisions = collisions + 1;
        }
    }

    println!(
        "Collisions: {} Expected: {}",
        collisions,
        iterations / hashing::SIZE
    );
}
