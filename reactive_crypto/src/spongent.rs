extern crate spongent_cpp_rs;

use spongent_cpp_rs::{spongent_wrap, spongent_unwrap, SpongentResult};
use crate::Error;


pub fn encrypt(plaintext : &[u8], key : &[u8], data : &[u8]) -> Result<Vec<u8>, Error> {
    let pl_len = plaintext.len();
    let sancus_security = key.len();
    let ad_len = data.len();

    //TODO: the encrypting function panics if plaintext and data are not multiple of 2 bytes
    if pl_len % 2 != 0 || ad_len % 2 != 0 {
        return Err(Error::IllegalArguments)
    }

    let mut ciphertext = vec![0u8; pl_len];
    let mut mac = vec![0u8; sancus_security];

    match spongent_wrap(key, data, plaintext, &mut ciphertext, &mut mac) {
        SpongentResult::Success => {
            ciphertext.extend_from_slice(&mac);
            Ok(ciphertext)
        },
        _                       => {
            Err(Error::EncryptionError)
        }
    }
}

pub fn decrypt(ciphertext : &[u8], key : &[u8], data : &[u8]) -> Result<Vec<u8>, Error> {
    let c_len = ciphertext.len();
    let sancus_security = key.len();
    let ad_len = data.len();

    if c_len < sancus_security {
        return Err(Error::KeySizeError)
    }

    let cipher_len = c_len - sancus_security;

    //TODO: the decrypting function panics if plaintext and data are not multiple of 2 bytes
    if c_len % 2 != 0 || ad_len % 2 != 0 {
        return Err(Error::IllegalArguments)
    }

    let cipher = &ciphertext[..cipher_len];
    let mac = &ciphertext[cipher_len..];

    let mut plaintext : Vec<u8> = vec![0u8; cipher_len];

    match spongent_unwrap(key, data, cipher, &mut plaintext, mac) {
        SpongentResult::Success => {
            Ok(plaintext)
        },
        _                       => {
            Err(Error::EncryptionError)
        }
    }
}
