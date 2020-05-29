extern crate spongent;

use spongent::{spongent_wrap, spongent_unwrap};
use crate::Error;


pub fn encrypt(plaintext : &[u8], key : &[u8], data : &[u8]) -> Result<Vec<u8>, Error> {
    let pl_len = plaintext.len();
    let sancus_security = key.len();
    let ad_len = data.len();

    //TODO: the encrypting function panics if plaintext and data are not multiple of 2 bytes
    if pl_len % 2 != 0 || ad_len % 2 != 0 {
        return Err(Error::IllegalArguments)
    }

    let mut ciphertext : Vec<u8> = Vec::with_capacity(pl_len + sancus_security + ad_len);
    ciphertext.extend_from_slice(data);
    ciphertext.extend_from_slice(plaintext);

    let mac = match spongent_wrap(key, data, plaintext, &mut ciphertext[ad_len..], false) {
        Ok(m) => m,
        Err(_) => return Err(Error::EncryptionError)
    };

    ciphertext.extend_from_slice(&mac);

    Ok(ciphertext)
}

pub fn decrypt(ciphertext : &[u8], key : &[u8], data : &[u8]) -> Result<Vec<u8>, Error> {
    let c_len = ciphertext.len();
    let sancus_security = key.len();
    let ad_len = data.len();

    let cipher_len = c_len - sancus_security - ad_len;

    //TODO: the decrypting function panics if plaintext and data are not multiple of 2 bytes
    if c_len % 2 != 0 || ad_len % 2 != 0 {
        return Err(Error::IllegalArguments)
    }

    if c_len != ad_len + cipher_len + sancus_security {
        return Err(Error::IllegalArguments)
    }

    let ad = &ciphertext[..ad_len];
    let cipher = &ciphertext[ad_len..ad_len + cipher_len];
    let mac = &ciphertext[ad_len + cipher_len..];

    if ad != data {
        // this to avoid potential replay attacks.
        return Err(Error::IllegalArguments)
    }

    let mut plaintext : Vec<u8> = Vec::with_capacity(cipher_len);
    plaintext.extend_from_slice(cipher);

    match spongent_unwrap(key, ad, cipher, mac, &mut plaintext) {
        Ok(_) =>  Ok(plaintext),
        Err(e) => {
            println!("{:?}", e);
            Err(Error::EncryptionError)
        }
    }
}
