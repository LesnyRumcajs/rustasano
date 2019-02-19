use std::collections::BTreeMap;

pub fn hamming_distance(first: &[u8], second: &[u8]) -> u32 {
    first
        .iter()
        .zip(second)
        .map(|x| x.0 ^ x.1)
        .fold(0, |acc, x| acc + x.count_ones())
}

pub fn count_same_blocks(ciphertext: &[u8], block_size: usize) -> u32 {
    let mut blocks_counter = BTreeMap::new();

    ciphertext
        .chunks(block_size)
        .for_each(|block| *blocks_counter.entry(block).or_insert(0u32) += 1);

    blocks_counter
        .values()
        .fold(0, |acc, count| acc + count - 1)
}

#[test]
fn hamming_distance_test() {
    assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37)
}

#[test]
fn count_same_blocks_test() {
    let same_blocks_data = b"1234test1234fooo";
    assert_eq!(count_same_blocks(same_blocks_data, 4), 1);

    let unique_blocks_data = b"12345678abcdefgh";
    assert_eq!(count_same_blocks(unique_blocks_data, 4), 0);
}
