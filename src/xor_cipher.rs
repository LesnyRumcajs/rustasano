use crate::fixed_xor::single_byte_xor;

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

pub fn crack_single_byte_xor(bytes: &[u8]) -> (Vec<u8>, i32, u8) {
    let mut result: Vec<u8> = Vec::new();
    let mut high_score = 0;
    let mut key = 0x00;

    for candidate in 0x00..0xFF {
        let plaintext = single_byte_xor(&bytes, candidate);
        let score = evaluate_english(&plaintext);
        if score > high_score {
            result = plaintext;
            high_score = score;
            key = candidate;
        }
    }

    (result, high_score, key)
}

#[test]
fn should_score_english_text() {
    let text = b"hello";
    assert_eq!(evaluate_english(text) > 0, true);
}

#[test]
fn should_punish_binary_text() {
    let text = [0x00, 0x01, 0xFF];
    assert_eq!(evaluate_english(&text) < 0, true);
}