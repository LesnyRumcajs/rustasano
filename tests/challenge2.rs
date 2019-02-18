/// Fixed XOR
/// https://cryptopals.com/sets/1/challenges/2
use rustasano::fixed_xor::fixed_xor_from_hex;

#[test]
fn should_pass_matasano_2() {
    let (first, second) = (
        "1c0111001f010100061a024b53535009181c",
        "686974207468652062756c6c277320657965",
    );
    let result = fixed_xor_from_hex(first, second);

    assert_eq!(result, "746865206b696420646f6e277420706c6179");
}
