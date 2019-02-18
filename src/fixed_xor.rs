use std::iter;

use crate::xor_cipher::crack_single_byte_xor;

pub fn fixed_xor_from_hex(first: &str, second: &str) -> String {
    let (first_bytes, second_bytes) = (hex::decode(first).unwrap(), hex::decode(second).unwrap());
    let result_byte = fixed_xor(&first_bytes, &second_bytes);

    hex::encode(result_byte)
}

pub fn fixed_xor(first: &[u8], second: &[u8]) -> Vec<u8> {
    first.iter().zip(second).map(|x| x.0 ^ x.1).collect()
}

pub fn single_byte_xor(data: &[u8], key: u8) -> Vec<u8> {
    iter::repeat(key).zip(data).map(|x| x.0 ^ x.1).collect()
}

pub fn single_byte_xor_detect(data: Vec<String>) -> Vec<u8> {
    data.iter()
        .map(|l| crack_single_byte_xor(&l))
        .max_by_key(|x| x.1)
        .expect("Unable to get max")
        .0
}

#[cfg(test)]
mod tests {
    use crate::file_utils::file_lines_to_vec;
    use crate::fixed_xor::*;

    #[test]
    fn should_pass_matasano_2() {
        let (first, second) = (
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c277320657965",
        );
        let result = fixed_xor_from_hex(first, second);

        assert_eq!(result, "746865206b696420646f6e277420706c6179");
    }

    #[test]
    fn fixed_xor_test() {
        assert_eq!(vec![1, 0], fixed_xor(&[0, 1], &[1, 1]));
        assert_eq!(vec![0x54, 0x9F], fixed_xor(&[0xFF, 0xAB], &[0xAB, 0x34]));
    }

    #[test]
    fn single_byte_xor_test() {
        let testee = vec![0x00, 0x01, 0xAA, 0xFF];

        assert_eq!(single_byte_xor(&testee, 0x00), testee);
        assert_eq!(single_byte_xor(&testee, 0x01), vec![0x01, 0x00, 0xAB, 0xFE]);
    }

    #[test]
    fn should_pass_matasano_4() {
        let input = file_lines_to_vec("res/set1/4.txt");
        assert_eq!(
            "Now that the party is jumping\n",
            String::from_utf8(single_byte_xor_detect(input)).unwrap()
        );
    }
}
