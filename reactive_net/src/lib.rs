//! # reactive_net
//!
//! `reactive_net` contains some utilities to manage network connections in an Authentic Execution environment
//! Messages are of three types: Message, Command, Result
//! The general format of a message is the following: [<identifier><length><payload>]
//! where `identifier` is different for each type, while the other fields are the same

mod result_message;
mod command_message;
mod functions;


pub use result_message::*;
pub use command_message::*;
pub use functions::*;


#[derive(Debug)]
pub enum Error {
    NetworkError,
    PayloadError,
    InvalidPayload,
    InternalError
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
        -> Result<(), std::fmt::Error> {
            write!(f, "{:?}", self)
        }
}

impl std::error::Error for Error {}

pub enum EntrypointID {
    SetKey,
    Attest,
    Disable,
    HandleInput,
    HandleHandler,
    UserDefined
}

impl EntrypointID {
    pub fn from_u16(value : u16) -> EntrypointID {
        match value {
            0 => EntrypointID::SetKey,
            1 => EntrypointID::Attest,
            2 => EntrypointID::Disable,
            3 => EntrypointID::HandleInput,
            4 => EntrypointID::HandleHandler,
            _ => EntrypointID::UserDefined
        }
    }
}
