use sgx_crypto::certificate::X509Cert;

#[test]
pub fn test_cert() {
    let mut cert = std::fs::read("tests/cert.pem").unwrap();
    cert.push(0);
    X509Cert::new_from_pem(&cert).unwrap();
}
