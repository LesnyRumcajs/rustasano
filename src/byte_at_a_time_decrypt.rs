use crate::aes_crypt::aes_ecb_encrypt;
use crate::aes_crypt::create_key;
use crate::crypto_utils::count_same_blocks;

const BLOCK_SIZE: usize = 16;

#[derive(Default)]
pub struct Oracle {
    key: [u8; BLOCK_SIZE],
    secret: Vec<u8>,
}

impl Oracle {
    pub fn new(secret: &[u8]) -> Oracle {
        Oracle {
            key: create_key(),
            secret: secret.to_vec(),
        }
    }

    pub fn encrypt(&self, input_data: &[u8]) -> Vec<u8> {
        let mut data_to_encrypt = input_data.to_vec();
        data_to_encrypt.extend(self.secret.iter());

        aes_ecb_encrypt(&data_to_encrypt, &self.key)
    }
}

pub fn detect_block_size() -> usize {
    let secret = b"Quidquid Latine dictum sit, altum videtur";
    let oracle = Oracle::new(secret);

    let input = vec![b'A'; 128];
    (2..=64usize)
        .find(|block_size| {
            input.len() / block_size - 1
                == count_same_blocks(&oracle.encrypt(&input), *block_size) as usize
        })
        .take()
        .expect("Couldn't find the block size")
}

pub fn simple_ecb_decryptor(secret: &[u8]) -> Vec<u8> {
    let oracle = Oracle::new(secret);
    let block_size = detect_block_size();

    let mut recovered_secret = Vec::new();

    for pos in 1..=16 {
        let input_len = block_size;
        let mut input = vec![b'A'; input_len];
        let reference_block = oracle.encrypt(&input[0..input_len - pos])[0..input_len].to_vec();

        input.splice((input_len - recovered_secret.len() - 1)..(input_len - 1), recovered_secret.iter().cloned());

        recovered_secret.push((0..=255u8).find(|candidate| {
            input[input_len - 1] = *candidate;
            reference_block == &oracle.encrypt(&input)[0..input_len]
        }).take().expect("Couldn't find the matching block"));
    }

//    let mut input = vec![b'A'; block_size];
//    let reference_block = oracle.encrypt(&input[0..block_size - 1])[0..block_size].to_vec();
//
//    recovered_secret.push((0..=255u8).find(|candidate| {
//        input[block_size - 1] = *candidate;
//        reference_block == &oracle.encrypt(&input)[0..block_size]
//    }).take().expect("Couldn't find the matching block"));
//
//    let mut input = vec![b'A'; block_size];
//
//    let reference_block = oracle.encrypt(&input[..block_size - 2])[0..block_size].to_vec();
//    input[block_size - 2] = recovered_secret[0];
//
//    recovered_secret.push((0..=255u8).find(|candidate| {
//        input[block_size - 1] = *candidate;
//        reference_block == &oracle.encrypt(&input)[0..block_size]
//    }).take().expect("Couldn't find the matching block"));

    recovered_secret
}

#[test]
fn detect_block_size_test() {
    assert_eq!(detect_block_size(), BLOCK_SIZE);
}

#[test]
fn simple_ecb_decryptor_single_block_test() {
    let secret = b"admin123456admin";
    assert_eq!(simple_ecb_decryptor(secret), secret);
}

#[test]
fn simple_ecb_decryptor_multiple_block_test() {
    let secret = b"admin123admin123";
    assert_eq!(simple_ecb_decryptor(secret), secret);
}