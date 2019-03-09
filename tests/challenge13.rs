use rustasano::aes_crypt;
use rustasano::ecb_cut_paste::{parse_kv, profile_for};
use std::str::from_utf8;

/// ECB cut-and-paste
/// https://cryptopals.com/sets/2/challenges/13

#[test]
fn should_pass_matasano13() {
    let key = aes_crypt::create_key();

    let admin_block = profile_for("aaaaaaaaaaadmin\x0B\x0B\x0B\x0B\x0B\x0B\x0B\x0B\x0B\x0B\x0B");
    let encrypted_admin_block = &aes_crypt::aes_ecb_encrypt(admin_block.as_bytes(), &key)
        [aes_crypt::AES_BLOCK_SIZE..2 * aes_crypt::AES_BLOCK_SIZE];

    let profile = profile_for("admin@bar.com");
    let mut encrypted_profile = aes_crypt::aes_ecb_encrypt(profile.as_bytes(), &key);

    encrypted_profile.splice(
        (encrypted_profile.len() - aes_crypt::AES_BLOCK_SIZE)..,
        encrypted_admin_block.to_vec(),
    );

    let decrypted_profile = aes_crypt::aes_ecb_decrypt(&encrypted_profile, &key);

    let result = parse_kv(from_utf8(&decrypted_profile).unwrap()).unwrap();
    assert_eq!(result["email"], "admin@bar.com");
    assert_eq!(result["uid"], "10");
    assert_eq!(result["role"], "admin");
}
