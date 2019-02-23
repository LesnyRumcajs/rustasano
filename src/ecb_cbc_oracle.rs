use crate::aes_crypt::aes_cbc_encrypt;
use crate::aes_crypt::aes_ecb_encrypt;
use crate::aes_crypt::create_key;
use crate::crypto_utils::count_same_blocks;
use rand::*;

const BLOCK_SIZE: usize = 16;

#[derive(Debug, PartialEq)]
pub enum BlockMode {
    ECB,
    CBC,
}

#[derive(Default)]
pub struct Oracle {
    key: [u8; BLOCK_SIZE],
}

impl Oracle {
    pub fn new() -> Oracle {
        Oracle { key: create_key() }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> (Vec<u8>, BlockMode) {
        let mut rng = rand::thread_rng();

        let prefix_count = rng.gen_range(5, 11);
        let mut data_to_encrypt = std::iter::repeat_with(|| rng.gen())
            .take(prefix_count)
            .collect::<Vec<u8>>();

        let postfix_count = rng.gen_range(5, 11);
        data_to_encrypt.extend(plaintext);
        data_to_encrypt.extend(
            std::iter::repeat_with(|| rng.gen())
                .take(postfix_count)
                .collect::<Vec<u8>>(),
        );

        if rng.gen() {
            (
                aes_cbc_encrypt(&data_to_encrypt, &self.key, &[0; BLOCK_SIZE]),
                BlockMode::CBC,
            )
        } else {
            (aes_ecb_encrypt(&data_to_encrypt, &self.key), BlockMode::ECB)
        }
    }
}

pub fn detect(ciphertext: &[u8]) -> BlockMode {
    if count_same_blocks(ciphertext, BLOCK_SIZE) > 0 {
        BlockMode::ECB
    } else {
        BlockMode::CBC
    }
}
