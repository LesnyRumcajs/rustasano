use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Ecb};

type Aes128Ecb = Ecb<Aes128, Pkcs7>;

pub fn aes_ecb_decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes128Ecb::new_var(key, Default::default()).unwrap();
    cipher.decrypt_vec(&ciphertext).unwrap()
}
