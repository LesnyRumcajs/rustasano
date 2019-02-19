/// Implement PKCS#7 padding
/// https://cryptopals.com/sets/2/challenges/9
use rustasano::pkcs7::Pkcs7;

fn should_pass_matasano9() {
    assert_eq!(
        "YELLOW SUBMARINE".as_bytes().to_vec().apply_pkcs7(16),
        "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes().to_vec()
    );
}
