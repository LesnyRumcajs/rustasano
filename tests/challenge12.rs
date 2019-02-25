use rustasano::byte_at_a_time_decrypt::simple_ecb_decryptor;
use rustasano::byte_at_a_time_decrypt::Oracle;

/// Byte-at-a-time ECB decryption (Simple)
/// https://cryptopals.com/sets/2/challenges/12

#[test]
fn should_pass_matasano12() {
    let input = base64::decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSB\
    yYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqIE5vLCBJIG\
    p1c3QgZHJvdmUgYnkK").unwrap();

    let oracle = Oracle::new(&input);

    assert_eq!(simple_ecb_decryptor(&oracle), input);
}
