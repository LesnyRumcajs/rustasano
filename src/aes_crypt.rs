use aes::Aes128;
use block_modes::block_padding::{Padding, Pkcs7, ZeroPadding};
use block_modes::{BlockMode, Ecb};
use rand::Rng;

use crate::fixed_xor::fixed_xor;
use crate::pkcs7::Pkcs7 as pkcs7_padding;

type Aes128Ecb = Ecb<Aes128, Pkcs7>;
type Aes128EcbZero = Ecb<Aes128, ZeroPadding>;

const AES_BLOCK_SIZE: usize = 16;

pub fn aes_ecb_decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes128Ecb::new_var(key, Default::default()).unwrap();
    cipher.decrypt_vec(&ciphertext).unwrap()
}

pub fn aes_cbc_encrypt(
    plaintext: &[u8],
    key: &[u8; AES_BLOCK_SIZE],
    iv: &[u8; AES_BLOCK_SIZE],
) -> Vec<u8> {
    let mut iv = iv.to_vec();

    plaintext
        .to_vec()
        .apply_pkcs7(AES_BLOCK_SIZE)
        .chunks(AES_BLOCK_SIZE)
        .map(|block| {
            let cipher = Aes128EcbZero::new_var(key, Default::default()).unwrap();
            let ciphertext_block = cipher.encrypt_vec(&fixed_xor(block, &iv));
            iv = ciphertext_block.clone();
            ciphertext_block
        })
        .flatten()
        .collect()
}

pub fn aes_cbc_decrypt(
    ciphertext: &[u8],
    key: &[u8; AES_BLOCK_SIZE],
    iv: &[u8; AES_BLOCK_SIZE],
) -> Vec<u8> {
    let mut result = Vec::new();

    let mut iv = iv.to_vec();

    ciphertext.chunks(AES_BLOCK_SIZE).for_each(|block| {
        let cipher = Aes128EcbZero::new_var(key, Default::default()).unwrap();
        let plaintext = fixed_xor(&cipher.decrypt_vec(&block).unwrap(), &iv);
        iv = block.to_vec();
        result.extend(plaintext);
    });

    Pkcs7::unpad(&result).unwrap().to_vec()
}

pub fn create_key() -> [u8; 16] {
    rand::thread_rng().gen::<[u8; AES_BLOCK_SIZE]>()
}

#[test]
fn create_key_len() {
    assert_eq!(create_key().len(), AES_BLOCK_SIZE);
}

#[test]
fn create_key_always_different() {
    assert_ne!(create_key(), create_key());
}

#[test]
fn encrypt_decrypt_test() {
    let key = b"YELLOW SUBMARINE";
    let iv = [0; AES_BLOCK_SIZE];

    let plaintext = b"Test123Test123Test123Test123Test123Test123";
    let ciphertext = aes_cbc_encrypt(plaintext, key, &iv);

    let result_plaintext = aes_cbc_decrypt(&ciphertext, key, &iv);
    assert_eq!(result_plaintext[..], plaintext[..]);
}
