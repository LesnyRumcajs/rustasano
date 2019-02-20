use aes::Aes128;

use block_modes::block_padding::{Padding, Pkcs7, ZeroPadding};
use block_modes::{BlockMode, Ecb};

use crate::fixed_xor::fixed_xor;

type Aes128Ecb = Ecb<Aes128, Pkcs7>;
type Aes128EcbZero = Ecb<Aes128, ZeroPadding>;

pub fn aes_ecb_decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes128Ecb::new_var(key, Default::default()).unwrap();
    cipher.decrypt_vec(&ciphertext).unwrap()
}

pub fn aes_cbc_decrypt(ciphertext: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
    let mut result = Vec::new();

    let mut iv = iv.to_vec();

    ciphertext.chunks(16).for_each(|block| {
        let cipher = Aes128EcbZero::new_var(key, Default::default()).unwrap();
        let plaintext = fixed_xor(&cipher.decrypt_vec(&block).unwrap(), &iv);
        iv = block.to_vec();
        result.extend(plaintext);
    });

    Pkcs7::unpad(&result).unwrap().to_vec()
}
