use anyhow::Result;
use ra_client::ClientRaContext;
use std::net::{TcpStream, TcpListener};
use std::io::Read;

use crate::error::Error;
use crate::MAX_HOST_SIZE;

pub fn start_client(port : u16) -> Result<()> {
    let listener = match TcpListener::bind(("0.0.0.0", port)) {
        Ok(l)   => l,
        Err(_)  => return Err(Error::BindPortError.into())
    };

    if cfg!(feature = "client_verbose") {
        println!("Listening on {}", port);
    }

    for stream in listener.incoming() {
        match stream {
            Ok(s) => if let Err(e) = handle_ra(s) {
                if cfg!(feature = "client_verbose") {
                    eprintln!("node_client error: {:?}", e);
                }
            },
            Err(_) => if cfg!(feature = "client_verbose") {
                    eprintln!("node_client error unwrapping the stream");
                }
        }
    }
    Ok(())
}

fn handle_ra(mut sp_stream : TcpStream) -> Result<()> {
    let mut port_buf = [0u8; 2];
    sp_stream.read_exact(&mut port_buf)?;
    let port = u16::from_be_bytes(port_buf);

    sp_stream.read_exact(&mut port_buf)?;
    let host_len = u16::from_be_bytes(port_buf) as usize;

    if host_len > MAX_HOST_SIZE {
        return  Err(Error::EnclaveHostTooLong.into())
    }

    let mut host_buf = vec!(0u8; host_len);
    sp_stream.read_exact(&mut host_buf)?;
    let host = String::from_utf8(host_buf)?;

    if cfg!(feature = "client_verbose") {
        println!("Enclave address: {}:{}", host, port);
    }

    let context = ClientRaContext::init()?;

    let mut enclave_stream = match TcpStream::connect((host, port)) {
        Ok(s)   => s,
        Err(_)  => return Err(Error::EnclaveConnectionError.into())
    };

    context.do_attestation(&mut enclave_stream, &mut sp_stream)?;
    Ok(())
}
