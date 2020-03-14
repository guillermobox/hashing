use hashing;

#[test]
fn test_pool_provides_hash_functions() {
    let size = 128;
    let f = hashing::IntegerHashFunction::new(size);
    let x: u32 = 2;

    assert!(f.hash(x) < size);
}
