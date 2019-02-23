use rustasano::ecb_cbc_oracle::detect;
/// An ECB/CBC detection oracle
/// https://cryptopals.com/sets/2/challenges/11
use rustasano::ecb_cbc_oracle::Oracle;

#[test]
fn should_pass_matasano11() {
    let input = [0; 128];

    for _ in 1..10 {
        let oracle = Oracle::new();
        let (ciphertext, actual_mode) = oracle.encrypt(&input);
        let detected_mode = detect(&ciphertext);
        assert_eq!(actual_mode, detected_mode);
    }
}
