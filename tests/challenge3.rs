/// Single-byte XOR cipher
/// https://cryptopals.com/sets/1/challenges/3
use rustasano::xor_cipher::crack_single_byte_xor;

#[test]
fn should_pass_matasano3() {
    let result = crack_single_byte_xor(
        &hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .unwrap(),
    );
    assert_eq!(
        String::from_utf8(result.0).unwrap(),
        "Cooking MC's like a pound of bacon"
    );
}
