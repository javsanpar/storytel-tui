use openssl::symm::{encrypt, Cipher};
use std::convert::TryInto;

const KEY: &[u8] = b"VQZBJ6TD8M9WBUWT";
const IV: &[u8] = b"joiwef08u23j341a";
const BLOCK_SIZE: u8 = 8;

fn pad(password: &str) -> String {
    let mut padded_password = String::from(password);

    let pass_length = password.chars().count() as u8;
    let module = pass_length % BLOCK_SIZE;
    if module != 0 {
        let pad_length = BLOCK_SIZE - module;

        padded_password = format!("{}{}", password,
                                      pad_length.to_string()
                                      .repeat(pad_length.try_into().unwrap())
                                      .as_str());
    }

    return padded_password;
}

pub fn encrypt_password(password: &str) -> String {
    let padded_password = pad(password);
    let cipher = Cipher::aes_128_cbc();
    let mut hex = String::new();

    let data = padded_password.as_bytes();
    let encrypted_data = encrypt(cipher, KEY, Some(IV), data).unwrap();

    for hex_pair in encrypted_data {
        hex = format!("{}{:02X}", hex, hex_pair);
    }

    return hex;
}

