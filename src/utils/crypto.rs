use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit, OsRng},
};
use base64::{engine::general_purpose, Engine};
use rand::RngCore;
use std::env;

/// Encrypt a value using AES-256-GCM
pub fn encrypt(value: &str) -> String {
    // Read key from env at runtime
    let key_str = env::var("ENCRYPTION_KEY")
        .expect("ENCRYPTION_KEY must be set and 32 bytes long");
    
    let key_bytes = key_str.as_bytes();
    assert_eq!(key_bytes.len(), 32, "ENCRYPTION_KEY must be 32 bytes");

    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Random 12-byte nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, value.as_bytes())
        .expect("encryption failure");

    // Combine nonce + ciphertext
    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);

    // Encode to base64
    general_purpose::STANDARD.encode(result)
}

/// Decrypt a value using AES-256-GCM
pub fn decrypt(encoded: &str) -> String {
    // Read key from env at runtime
    let key_str = env::var("ENCRYPTION_KEY")
        .expect("ENCRYPTION_KEY must be set and 32 bytes long");
    
    let key_bytes = key_str.as_bytes();
    assert_eq!(key_bytes.len(), 32, "ENCRYPTION_KEY must be 32 bytes");

    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Decode base64
    let decoded = general_purpose::STANDARD.decode(encoded)
        .expect("base64 decode failed");

    // Split nonce and ciphertext
    let (nonce_bytes, cipher_text) = decoded.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt
    let plaintext = cipher.decrypt(nonce, cipher_text)
        .expect("decryption failure");

    String::from_utf8(plaintext).expect("utf8 decode failure")
}
