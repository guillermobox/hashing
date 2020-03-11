use hashing::{self, Hashable};

#[test]
fn the_right_function_hashes_u32() {
    let f = hashing::UniversalHashFunction::<u64>::new();
    let x: u32 = 5;

    assert!(x.hash_with(&f) < hashing::SIZE);
}

#[test]
fn the_right_function_hashes_string() {
    let f = hashing::UniversalHashFunction::<u32>::new();
    let x = "any string will be hashable";

    assert!(x.hash_with(&f) < hashing::SIZE);
}
