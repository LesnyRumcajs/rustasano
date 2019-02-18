use base64;
use hex;

pub fn hex2base64(hex: &str) -> String {
    base64::encode(&hex::decode(hex).unwrap())
}
