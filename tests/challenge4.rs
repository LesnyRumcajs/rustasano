/// Detect single-character XOR
/// https://cryptopals.com/sets/1/challenges/4
use rustasano::file_utils::file_lines_to_vec;
use rustasano::fixed_xor::single_byte_xor_detect;

#[test]
fn should_pass_matasano_4() {
    let input = file_lines_to_vec("res/set1/4.txt");
    assert_eq!(
        "Now that the party is jumping\n",
        String::from_utf8(single_byte_xor_detect(input)).unwrap()
    );
}
