use anyhow::Result;
use std::net::TcpListener;
use ra_enclave::EnclaveRaContext;

use crate::error::Error;

pub fn do_attestation(port : u16, sp_vkey : &str) -> Result<Vec<u8>> {
    if cfg!(feature = "enclave_verbose") {
        println!("Waiting for attestation on {}", port);
    }

    let listener = TcpListener::bind(("0.0.0.0", port))?;
    let mut stream = listener.accept()?.0;

    if cfg!(feature = "enclave_verbose") {
        println!("Connected to ra_client");
    }

    let context = match EnclaveRaContext::init(sp_vkey) {
        Ok(c) => c,
        Err(e) => {
            if cfg!(feature = "enclave_verbose") {
                println!("ERROR: {}", e);
            }
            return Err(Error::FailedToInitEnclaveRaContext.into());
        }
    };

    let result = match context.do_attestation(&mut stream) {
        Ok(r) => r,
        Err(e) => {
            if cfg!(feature = "enclave_verbose") {
                println!("ERROR: {}", e);
            }
            
            return Err(Error::AttestationFailed(e).into());
        }
    };

    if cfg!(feature = "enclave_verbose") {
        println!("Remote attestation succeeded");
    }

    Ok(result.1.to_vec())
}