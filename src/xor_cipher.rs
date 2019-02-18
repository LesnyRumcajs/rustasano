use crate::fixed_xor::single_byte_xor;
use std::str;

pub fn evaluate_english(data: &[u8]) -> i32 {
    data.iter().fold(0, |acc, &ch| {
        if "etain shrdlu".contains((ch as char).to_ascii_lowercase()) {
            acc + 5
        } else if ch.is_ascii_alphabetic() {
            acc + 1
        } else {
            acc - 5
        }
    })
}

pub fn crack_single_byte_xor(hex_encoded: &str) -> (Vec<u8>, i32) {
    let bytes = hex::decode(hex_encoded).unwrap();

    let mut result: Vec<u8> = Vec::new();
    let mut high_score = 0;

    for candidate in 0x00..0xFF {
        let plaintext = single_byte_xor(&bytes, candidate);
        let score = evaluate_english(&plaintext);
        if score > high_score {
            result = plaintext;
            high_score = score;
        }
    }

    (result, high_score)
}

#[test]
fn should_score_english_text() {
    let text = b"hello";
    assert_eq!(evaluate_english(text), 21);
}

#[test]
fn should_pass_matasano3() {
    let result = crack_single_byte_xor(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    );
    assert_eq!(
        String::from_utf8(result.0).unwrap(),
        "Cooking MC's like a pound of bacon"
    );
}
