/// Implement repeating-key XOR
/// https://cryptopals.com/sets/1/challenges/5

use rustasano::repeating_key_xor::repeating_key_xor;

#[test]
fn should_pass_matasano_5() {
    let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";
    let expected_hex = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    let result_hex = hex::encode(repeating_key_xor(input, key));
    assert_eq!(expected_hex, result_hex);
}
