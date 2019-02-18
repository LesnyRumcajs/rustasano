pub fn hamming_distance(first: &[u8], second: &[u8]) -> u32 {
    first
        .iter()
        .zip(second)
        .map(|x| x.0 ^ x.1)
        .fold(0, |acc, x| acc + x.count_ones())
}

#[test]
fn hamming_distance_test() {
    assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37)
}
