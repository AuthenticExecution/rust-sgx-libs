use sgx_crypto::signature::{SigningKey, VerificationKey};

#[test]
pub fn test_private_key() {
    let mut key = std::fs::read("tests/privkey.pem").unwrap();
    key.push(0);
    SigningKey::new(&key, None).unwrap();
}

#[test]
pub fn test_public_key() {
    let mut key = std::fs::read("tests/pubkey.pem").unwrap();
    key.push(0);
    VerificationKey::new(&key).unwrap();
}
