extern crate aes_gcm;

use aes_gcm::Aes128Gcm as AesGcm; // Or `Aes256Gcm`
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray, Payload};

use crate::Error;


pub fn encrypt(plaintext : &[u8], key : &[u8], nonce : u16, data : &[u8]) -> Result<Vec<u8>, Error> {
    let key_arr = GenericArray::clone_from_slice(key);
    let nonce = create_nonce(nonce);
    let nonce_arr = GenericArray::from_slice(&nonce);

    let aes = AesGcm::new(key_arr);
    match aes.encrypt(nonce_arr, Payload{msg : plaintext, aad : data}) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::EncryptionError)
    }
}

pub fn decrypt(ciphertext : &[u8], key : &[u8], nonce : u16, data : &[u8]) -> Result<Vec<u8>, Error> {
    let key_arr = GenericArray::clone_from_slice(key);
    let nonce = create_nonce(nonce);
    let nonce_arr = GenericArray::from_slice(&nonce);

    let aes = AesGcm::new(key_arr);
    match aes.decrypt(nonce_arr, Payload{msg : ciphertext, aad : data}) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::EncryptionError)
    }
}

fn create_nonce(nonce : u16) -> Vec<u8> {
    let mut arr = vec![0u8; 10];
    arr.extend_from_slice(&nonce.to_be_bytes());

    arr
}
