extern crate aes_gcm;

use aes_gcm::Aes128Gcm as AesGcm; // Or `Aes256Gcm`
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray, Payload};

use crate::Error;


pub fn encrypt(plaintext : &[u8], key : &[u8], data : &[u8]) -> Result<Vec<u8>, Error> {
    let key_arr = GenericArray::clone_from_slice(key);
    let nonce = [0u8; 12]; // not used: it is included in associated data
    let nonce_arr = GenericArray::from_slice(&nonce);

    let aes = AesGcm::new(&key_arr);
    match aes.encrypt(nonce_arr, Payload{msg : plaintext, aad : data}) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::EncryptionError)
    }
}

pub fn decrypt(ciphertext : &[u8], key : &[u8], data : &[u8]) -> Result<Vec<u8>, Error> {
    let key_arr = GenericArray::clone_from_slice(key);
    let nonce = [0u8; 12]; // not used: it is included in associated data
    let nonce_arr = GenericArray::from_slice(&nonce);

    let aes = AesGcm::new(&key_arr);
    match aes.decrypt(nonce_arr, Payload{msg : ciphertext, aad : data}) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::EncryptionError)
    }
}
