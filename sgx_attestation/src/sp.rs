use sgx_crypto::certificate::X509Cert;
use sgx_crypto::signature::SigningKey;
use std::net::TcpStream;
use anyhow::Result;
use ra_sp::SpRaContext;
use std::io::Write;
use sgxs::sigstruct::Sigstruct;
use ra_sp::SpConfig;

use crate::error::Error;
use crate::MAX_HOST_SIZE;

pub fn attest_enclave(host : &str, port : u16, aesm_host : &str, aesm_port : u16,
    key : &[u8], cert : &[u8], config : &[u8], sigstruct : &[u8]) -> Result<Vec<u8>> {
        let key = SigningKey::new(key, None)?;
        let cert = X509Cert::new_from_pem(cert)?;
        let config : SpConfig = serde_json::from_slice(config)?;
        let sigstruct = match Sigstruct::try_copy_from(sigstruct) {
            Some(c)     => c,
            None        => return Err(Error::SigstructDeserializationError.into())
        };

        let context = SpRaContext::init_from(config, sigstruct, key, cert)?;
        let mut stream = TcpStream::connect((aesm_host, aesm_port))?;

        let port_buf = port.to_be_bytes();
        stream.write(&port_buf)?;

        let host_len = host.len();

        if host_len > MAX_HOST_SIZE {
            return Err(Error::EnclaveHostTooLong.into())
        }

        let host_len_buf = (host_len as u16).to_be_bytes();
        stream.write(&host_len_buf)?;
        stream.write(host.as_bytes())?;

        let result = context.do_attestation(&mut stream)?;

        // TODO is this conversion necessary?
        Ok(result.master_key.to_vec())
}
