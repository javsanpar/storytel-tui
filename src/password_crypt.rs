use openssl::symm::{encrypt, Cipher};

const KEY: &[u8] = b"VQZBJ6TD8M9WBUWT";
const IV: &[u8] = b"joiwef08u23j341a";

pub fn encrypt_password(password: &str) -> String {
    let cipher = Cipher::aes_128_cbc();
    let mut hex = String::new();

    let data = password.as_bytes();
    let encrypted_data = encrypt(cipher, KEY, Some(IV), data).unwrap();

    for hex_pair in encrypted_data {
        hex = format!("{}{:02X}", hex, hex_pair);
    }

    return hex;
}
