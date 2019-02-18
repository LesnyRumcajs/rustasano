use std::iter::repeat;

pub fn repeating_key_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    input
        .iter()
        .zip(key.iter().cycle())
        .map(|x| x.0 ^ x.1)
        .collect()
}
