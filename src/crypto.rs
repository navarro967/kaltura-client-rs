/// This module provides cryptographic utilities for the application.
///
/// It includes functions for SHA-1 hashing and AES encryption.
///
/// # Example
///
/// ```
/// use kaltura_client_rs::crypto::{sha1, aes_encrypt, AES_KEY_LEN, AES_IV};
///
/// let data = vec![1, 2, 3, 4, 5];
/// let key = vec![0; AES_KEY_LEN];
///
/// let hashed_data = sha1(&data);
/// let encrypted_data = aes_encrypt(&mut data.clone(), &key, &AES_IV);
/// ```
///
/// # Note
///
/// This module uses the `aes` and `sha1` crates for encryption and hashing respectively.


use aes::{
    self,
    cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit},
};

use sha1::{Digest, Sha1};


type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

/// The length of the AES key in bytes.
pub const AES_KEY_LEN: usize = 16;
/// The size of the AES block in bytes.
pub const AES_BLOCK_SIZE: usize = 16;
/// The initialization vector (IV) for the AES encryption.
pub const AES_IV: [u8; AES_KEY_LEN] = [0x22; AES_KEY_LEN];

/// Computes the SHA-1 hash of the given data.
///
/// # Arguments
///
/// * `data` - The data to hash.
pub fn sha1(data: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let sig = hasher.finalize();
    sig.to_vec()
}

/// Encrypts the given data using AES encryption.
///
/// # Arguments
///
/// * `data` - The data to encrypt.
/// * `key` - The encryption key.
/// * `iv` - The initialization vector.
///
/// # Returns
/// The encrypted data.
/// 
/// # Example
/// ```
/// use kaltura_client_rs::crypto::{sha1, aes_encrypt, AES_KEY_LEN, AES_IV};
///
/// let data = vec![1, 2, 3, 4, 5];
/// let key = vec![0; AES_KEY_LEN];
///
/// let hashed_data = sha1(&data);
/// let encrypted_data = aes_encrypt(&mut data.clone(), &key, &AES_IV);
/// ```
pub fn aes_encrypt(data: &mut Vec<u8>, key: &Vec<u8>, iv: &[u8]) -> Vec<u8> {
    let length = data.len();
    let key = &key[..AES_KEY_LEN];

    if data.len() % AES_BLOCK_SIZE != 0 {
        let padding = AES_BLOCK_SIZE - (data.len() % AES_BLOCK_SIZE);
        data.extend(vec![0 as u8; padding]);
    }

    match Aes128CbcEnc::new(key.into(), iv.into())
        .encrypt_padded_mut::<aes::cipher::block_padding::ZeroPadding>(data, length)
    {
        Ok(cipher) => cipher.to_vec(),
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
        }
    }
}

/// Decrypts the given data using AES encryption.
///
/// # Arguments
///
/// * `data` - The data to encrypt.
/// * `key` - The encryption key.
/// * `iv` - The initialization vector.
pub fn aes_decrypt(data: &[u8], key: &str, iv: &[u8]) -> Vec<u8> {
    let mut block = data.to_vec();
    let cipher = Aes128CbcDec::new(key.as_bytes().into(), iv.into())
        .decrypt_padded_mut::<aes::cipher::block_padding::ZeroPadding>(&mut block)
        .unwrap();
    cipher.to_vec()
}