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

pub fn detect_ecb_block_size() -> usize {
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

pub fn simple_ecb_decryptor(oracle: &Oracle) -> Vec<u8> {
    let block_size = detect_ecb_block_size();
    let secret_len = recover_secret_len(&oracle, block_size);

    let mut recovered_secret = Vec::new();
    for pos in 0..secret_len {
        let block_count = recovered_secret.len() / block_size;

        let input_len = block_size;
        let mut input = vec![b'A'; input_len];
        let offset = pos % 16 + 1;

        let ciphertext = oracle.encrypt(&input[offset..]);
        let reference_block =
            ciphertext[block_count * block_size..(block_count + 1) * block_size].to_vec();

        if recovered_secret.len() < block_size {
            input.splice(
                (input_len - recovered_secret.len() - 1)..(input_len - 1),
                recovered_secret.iter().cloned(),
            );
        } else {
            input = recovered_secret.to_vec();
            input.push(0);
        }

        recovered_secret.push(
            (0..=255u8)
                .find(|candidate| {
                    let n = input.len() - 1;

                    input[n] = *candidate;
                    let to_encrypt = &input[input.len() - block_size..];
                    let ciphertext = oracle.encrypt(to_encrypt);
                    reference_block == &ciphertext[0..block_size]
                })
                .take()
                .expect("Couldn't find the matching block"),
        );
    }

    recovered_secret
}

fn recover_secret_len(oracle: &Oracle, block_size: usize) -> usize {
    let secret_len = oracle.encrypt(&b""[..]).len();

    secret_len
        - (1..=block_size)
            .find(|&x| secret_len != oracle.encrypt(&vec![0; x]).len())
            .take()
            .expect("Couldn't recover secret length")
}

#[test]
fn detect_block_size_test() {
    assert_eq!(detect_ecb_block_size(), BLOCK_SIZE);
}

#[test]
fn simple_ecb_decryptor_single_block_test() {
    let oracle = Oracle::new(b"admin123456admin");
    assert_eq!(simple_ecb_decryptor(&oracle), oracle.secret);
}

#[test]
fn simple_ecb_decryptor_multiple_block_test() {
    let oracle = Oracle::new(b"Quidquid Latine dictum sit, altum videtur.");
    assert_eq!(simple_ecb_decryptor(&oracle), oracle.secret);
}
