/// Convert hex to base64
/// https://cryptopals.com/sets/1/challenges/1
use rustasano::hex_utils::hex2base64;

#[test]
fn should_pass_matasano_1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert_eq!(
        hex2base64(input),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
}
