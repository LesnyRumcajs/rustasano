use base64;
use hex;

pub fn hex2base64(hex: &str) -> String {
    base64::encode(&hex::decode(hex).unwrap())
}

#[test]
fn should_pass_matasano_1() {
    let testee = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert_eq!(
        hex2base64(testee),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
}
