/// Detect AES in ECB mode
/// https://cryptopals.com/sets/1/challenges/8

use rustasano::crypto_utils::count_same_blocks;
use rustasano::file_utils::file_lines_to_vec;

fn should_pass_matasano8() {
    let ciphertexts = file_lines_to_vec("tests/res/8.txt");

    let expected_ecb_count = 1usize;
    let aes_block_size = 16usize;

    let result_ecb_count = ciphertexts
        .iter()
        .map(|ciphertext| count_same_blocks(&hex::decode(&ciphertext).unwrap(), aes_block_size))
        .filter(|&same_blocks| same_blocks > 0)
        .count();

    assert_eq!(expected_ecb_count, result_ecb_count);
}
