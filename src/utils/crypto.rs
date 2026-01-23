use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key as AesGcmKey, Nonce,
};
use aes_gcm_siv::{
    aead::{Aead as SivAead, KeyInit as SivKeyInit},
    Aes256GcmSiv, Nonce as SivNonce,
};
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;
use std::env;

/// === AES-256-GCM (random nonce) - for general encryption ===
/// Produces different output every time (even for same input)

/// Encrypt using AES-256-GCM with random nonce
pub fn encrypt(value: &str) -> String {
    let key_str = env::var("ENCRYPTION_KEY_NON_DETERMINISTIC")
        .expect("ENCRYPTION_KEY environment variable must be set");

    let key_bytes = key_str.as_bytes();
    if key_bytes.len() != 32 {
        panic!("ENCRYPTION_KEY must be exactly 32 bytes long");
    }

    let key = AesGcmKey::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Generate random 12-byte nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, value.as_bytes())
        .expect("AES-GCM encryption failed");

    // Prepend nonce → (nonce || ciphertext)
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);

    general_purpose::STANDARD.encode(result)
}

/// Decrypt AES-256-GCM ciphertext (expects nonce prepended)
pub fn decrypt(encoded: &str) -> String {
    let key_str = env::var("ENCRYPTION_KEY_NON_DETERMINISTIC")
        .expect("ENCRYPTION_KEY environment variable must be set");

    let key_bytes = key_str.as_bytes();
    if key_bytes.len() != 32 {
        panic!("ENCRYPTION_KEY must be exactly 32 bytes long");
    }

    let key = AesGcmKey::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    let decoded = general_purpose::STANDARD
        .decode(encoded)
        .expect("Base64 decode failed");

    if decoded.len() < 12 {
        panic!("Invalid ciphertext: too short");
    }

    let (nonce_bytes, ciphertext) = decoded.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .expect("AES-GCM decryption failed");

    String::from_utf8(plaintext).expect("Invalid UTF-8 after decryption")
}

/// === AES-256-GCM-SIV (deterministic / nonce-misuse-resistant) ===
/// Same plaintext → always same ciphertext
/// Ideal for database uniqueness constraints, searchable encryption, deduplication

type DeterministicCipher = Aes256GcmSiv;

/// Encrypt using AES-256-GCM-SIV (deterministic)
pub fn encrypt_deterministic(value: &str) -> String {
    let key_str = env::var("ENCRYPTION_KEY")
        .expect("ENCRYPTION_KEY environment variable must be set");

    // Assuming ENCRYPTION_KEY is **base64-encoded** 32-byte key
    // If it's raw bytes or hex, change decoding accordingly
    let key_bytes = general_purpose::STANDARD
        .decode(&key_str)
        .expect("ENCRYPTION_KEY must be valid base64 (32 bytes after decode)");

    if key_bytes.len() != 32 {
        panic!("ENCRYPTION_KEY must decode to exactly 32 bytes");
    }

    let key = aes_gcm_siv::Key::<DeterministicCipher>::from_slice(&key_bytes);
    let cipher = DeterministicCipher::new(key);

    // Nonce is ignored in deterministic usage of GCM-SIV
    // We use all-zero nonce (common convention)
    let nonce = SivNonce::from_slice(&[0u8; 12]);

    let ciphertext = cipher
        .encrypt(nonce, value.as_bytes())
        .expect("AES-GCM-SIV encryption failed");

    // No need to store nonce → just the ciphertext + tag
    general_purpose::STANDARD.encode(ciphertext)
}

/// Decrypt AES-256-GCM-SIV deterministic ciphertext
pub fn decrypt_deterministic(encoded: &str) -> String {
    let key_str = env::var("ENCRYPTION_KEY")
        .expect("ENCRYPTION_KEY environment variable must be set");

    let key_bytes = general_purpose::STANDARD
        .decode(&key_str)
        .expect("ENCRYPTION_KEY must be valid base64 (32 bytes after decode)");

    if key_bytes.len() != 32 {
        panic!("ENCRYPTION_KEY must decode to exactly 32 bytes");
    }

    let key = aes_gcm_siv::Key::<DeterministicCipher>::from_slice(&key_bytes);
    let cipher = DeterministicCipher::new(key);

    let decoded = general_purpose::STANDARD
        .decode(encoded)
        .expect("Base64 decode failed");

    let nonce = SivNonce::from_slice(&[0u8; 12]);

   let plaintext = cipher
    .decrypt(nonce, decoded.as_slice())   // & [u8]
    .expect("...");

    String::from_utf8(plaintext).expect("Invalid UTF-8 after decryption")
}