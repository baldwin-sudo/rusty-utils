use hex::ToHex;
use hmac::Hmac;
use pbkdf2::pbkdf2;

use sha2::{Digest, Sha256};
pub fn derive_key_from_master(master_password: String, salt: String) -> [u8; 16] {
    let mut buf = [0u8; 16];
    pbkdf2::<Hmac<Sha256>>(
        master_password.as_bytes(),
        salt.as_bytes(),
        600_000,
        &mut buf,
    )
    .expect("HMAC can be initialized with any key length");
    return buf;
}
pub fn ceasar_encrypt(plaintext: &str, shift: u8) -> String {
    let shift = shift % 26; // Ensure shift is within 0-25
    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { 'A' } else { 'a' };
                let offset = (c as u8 - base as u8 + shift) % 26;
                (base as u8 + offset) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn ceasar_decrypt(ciphertext: &str, shift: u8) -> String {
    let shift = shift % 26; // Ensure shift is within 0-25
    ceasar_encrypt(ciphertext, 26 - shift) // Decrypt by shifting in the opposite direction
}
pub fn vigenere_encrypt(plaintext: &str, keyword: &str) -> String {
    let keyword = keyword.to_uppercase();
    let keyword_bytes = keyword.as_bytes();
    let mut encrypted_text = String::with_capacity(plaintext.len());

    for (i, &byte) in plaintext.to_uppercase().as_bytes().iter().enumerate() {
        if byte.is_ascii_alphabetic() {
            let key_byte = keyword_bytes[i % keyword_bytes.len()];
            let offset = if byte.is_ascii_uppercase() {
                b'A'
            } else {
                b'a'
            };
            let encrypted_byte = (byte - offset + (key_byte - b'A')) % 26 + offset;
            encrypted_text.push(encrypted_byte as char);
        } else {
            // Keep non-alphabetic characters unchanged
            encrypted_text.push(byte as char);
        }
    }

    encrypted_text
}

pub fn vigenere_decrypt(ciphertext: &str, keyword: &str) -> String {
    let keyword = keyword.to_uppercase();
    let keyword_bytes = keyword.as_bytes();
    let mut decrypted_text = String::with_capacity(ciphertext.len());

    for (i, &byte) in ciphertext.to_uppercase().as_bytes().iter().enumerate() {
        if byte.is_ascii_alphabetic() {
            let key_byte = keyword_bytes[i % keyword_bytes.len()];
            let offset = if byte.is_ascii_uppercase() {
                b'A'
            } else {
                b'a'
            };
            let decrypted_byte = (byte - offset + 26 - (key_byte - b'A')) % 26 + offset;
            decrypted_text.push(decrypted_byte as char);
        } else {
            // Keep non-alphabetic characters unchanged
            decrypted_text.push(byte as char);
        }
    }

    decrypted_text
}

pub fn hash_master(master_password: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(master_password);
    let result = hasher.finalize();
    // to convert the hex into String
    let hex_string_sha512 = result.encode_hex::<String>();
    return hex_string_sha512;
}
pub fn compare_hashes(hash1: String, hash2: String) -> bool {
    return hash1 == hash2;
}
