//! # reactive_crypto
//!
//! `reactive_crypto` contains some crypto utilities used in an Authentic Execution environment

mod aes;
mod spongent;


/// This enum which encryption types are supported
#[derive(Debug, Clone)]
pub enum Encryption {
    Aes,
    Spongent
}

impl Encryption {
    /// Converts from &str to Option<Encryption>
    /// the input string is converted to lowercase before checking
    /// "Aes"      : Encryption::Aes
    /// "spongent"  : Encryption::Spongent
    pub fn from_str(enc : &str) -> Option<Encryption> {
        let lower = enc.to_lowercase();

        match &*lower {
            "aes"      => Some(Encryption::Aes),
            "spongent"  => Some(Encryption::Spongent),
            _           => None
        }
    }

    /// Converts from u8 to Option<Encryption>
    /// 0: Encryption::Aes
    /// 1: Encryption::Spongent
    pub fn from_u8(enc : u8) -> Option<Encryption> {
        match enc {
            0           => Some(Encryption::Aes),
            1           => Some(Encryption::Spongent),
            _           => None
        }
    }
}

#[derive(Debug)]
pub enum Error {
    EncryptionError,
    InternalError,
    NotImplemented,
    IllegalArguments,
    KeySizeError
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
        -> Result<(), std::fmt::Error> {
            write!(f, "{:?}", self)
        }
}


/// Encrypts `plaintext` using `key`, and `data`, with the chosen encryption type
/// Returns Ok(ciphertext) if everything went well
/// The return Vec will have the following format: [cipher - mac]
/// where cipher has the same length of the plaintext, and mac is 16 bytes
pub fn encrypt(plaintext : &[u8], key : &[u8], data : &[u8], encryption : &Encryption) -> Result<Vec<u8>, Error> {
    match encryption {
        Encryption::Aes => aes::encrypt(plaintext, key, data),
        Encryption::Spongent => spongent::encrypt(plaintext, key, data),
    }
}


/// Decrypts `ciphertext` using `key`, and `data`, with the chosen encryption type
/// Returns Ok(plaintext) if everything went well
/// `ciphertext` must have the format [cipher - mac]
/// where cipher is the encrypted data
pub fn decrypt(ciphertext : &[u8], key : &[u8], data : &[u8], encryption : &Encryption) -> Result<Vec<u8>, Error> {
    match encryption {
        Encryption::Aes => aes::decrypt(ciphertext, key, data),
        Encryption::Spongent => spongent::decrypt(ciphertext, key, data),
    }
}


#[cfg(test)]
mod tests {
    use super::{Encryption, encrypt, decrypt};

    fn test_generic(enc : Encryption, security_bytes : usize) {
        let key = b"16-bytes sec key";
        let plaintext = b"Hello world!";
        let data = [1u8, 2u8];

        let ciphertext = encrypt(plaintext, &key[..security_bytes], &data, &enc).unwrap();
        let plaintext_dec = decrypt(&ciphertext, &key[..security_bytes], &data, &enc).unwrap();

        assert_eq!(plaintext, &plaintext_dec[..]);
    }

    #[test]
    fn test_aes() {
        test_generic(Encryption::Aes, 16);
    }

    #[test]
    fn test_spongent() {
        test_generic(Encryption::Spongent, 8);
    }
}
