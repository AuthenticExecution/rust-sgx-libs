use crate::*;

use std::net::TcpStream;
use std::io::prelude::*;
use std::convert::TryFrom;

/// Read a message from stream
///
/// message format is: [len u16 - payload]
pub fn read_message(stream: &mut TcpStream) -> Result<Vec<u8>, Error> {
    let mut buf : [u8; 2] = [0; 2];

    if let Err(_) = stream.read_exact(&mut buf) {
        return Err(Error::NetworkError);
    }

    let size = bytes_to_u16(&buf);
    if size == 0 {
        return Ok(Vec::new());
    }

    let mut payload : Vec<u8> = Vec::with_capacity(size as usize);

    let mut buf : [u8; 1024] = [0; 1024];
    let mut size_left = size as usize;
    loop {
        let min = std::cmp::min(size_left, 1024);

        if let Err(_) = stream.read_exact(&mut buf[..min]) {
            return Err(Error::PayloadError);
        }

        payload.extend_from_slice(&buf[..min]);

        size_left -= min;

        if size_left == 0 {
            break;
        }
    }

    Ok(payload)
}


/// Write a message to stream
///
/// message format is: [len u16 - payload]
pub fn write_message(stream : &mut TcpStream, data : &[u8]) -> Result<(), Error> {
    let payload_len = match u16::try_from(data.len()) {
        Ok(l)  => l,
        Err(_) => return Err(Error::InvalidPayload)
    };

    let mut payload : Vec<u8> = Vec::with_capacity(payload_len as usize + 2);

    payload.extend_from_slice(&payload_len.to_be_bytes());
    payload.extend_from_slice(data);

    write(stream, &payload)
}


/// Read a ResultMessage from stream
///
/// message format is: [code u8 - len u16 - payload]
pub fn read_result(stream : &mut TcpStream) -> Result<ResultMessage, Error> {
    let mut buf : [u8; 1] = [0; 1];
    if let Err(_) = stream.read_exact(&mut buf) {
        return Err(Error::NetworkError);
    }

    let code = ResultCode::from_u8(buf[0]);

    match read_message(stream) {
        Ok(payload) if payload.len() > 0 => Ok(ResultMessage::new(code, Some(payload))),
        Ok(_)                            => Ok(ResultMessage::new(code, None)),
        Err(msg)                         => Err(msg)
    }
}


/// Write a ResultMessage to stream
///
/// message format is: [code u8 - len u16 - payload]
pub fn write_result(stream : &mut TcpStream, result : &ResultMessage) -> Result<(), Error> {
    let data = result.get_payload();

    let payload_len = match data {
        Some(p) => match u16::try_from(p.len()) {
            Ok(l)  => l,
            Err(_) => return Err(Error::InvalidPayload)
        },
        None => 0
    };

    let mut payload : Vec<u8> = Vec::with_capacity(payload_len as usize + 3);

    payload.push(result.get_code_u8());
    payload.extend_from_slice(&payload_len.to_be_bytes());

    if payload_len != 0 {
        payload.extend_from_slice(data.unwrap());
    }

    write(stream, &payload)
}


/// Read a CommandMessage from stream
///
/// message format is: [command u16 - len u16 - payload]
pub fn read_command(stream : &mut TcpStream) -> Result<CommandMessage, Error> {
    let mut buf : [u8; 2] = [0; 2];
    if let Err(_) = stream.read_exact(&mut buf) {
        return Err(Error::NetworkError);
    }

    let code = bytes_to_u16(&buf);
    let code = match CommandCode::from_u16(code) {
        Some(c) => c,
        None    => return Err(Error::InvalidPayload)
    };

    match read_message(stream) {
        Ok(payload) if payload.len() > 0 => Ok(CommandMessage::new(code, Some(payload))),
        Ok(_)                            => Ok(CommandMessage::new(code, None)),
        Err(msg)                         => Err(msg)
    }
}


/// Write a CommandMessage to stream
///
/// message format is: [command u16 - len u16 - payload]
pub fn write_command(stream : &mut TcpStream, command : &CommandMessage) -> Result<(), Error> {
    let data = command.get_payload();

    let payload_len = match data {
        Some(p) => match u16::try_from(p.len()) {
            Ok(l)  => l,
            Err(_) => return Err(Error::InvalidPayload)
        },
        None => 0
    };

    let mut payload : Vec<u8> = Vec::with_capacity(payload_len as usize + 4);

    payload.extend_from_slice(&command.get_code_u16().to_be_bytes());
    payload.extend_from_slice(&payload_len.to_be_bytes());

    if payload_len != 0 {
        payload.extend_from_slice(data.unwrap());
    }

    write(stream, &payload)
}


fn bytes_to_u16(buf : &[u8]) -> u16 {
    u16::from_be_bytes([buf[0], buf[1]])
}


fn write(stream : &mut TcpStream, payload : &[u8]) -> Result<(), Error> {
    match stream.write(&payload) {
        Ok(_b) if _b == payload.len() => Ok(()),
        _                             => Err(Error::NetworkError),
    }
}

#[cfg(test)]
mod tests {
    //TODO
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
