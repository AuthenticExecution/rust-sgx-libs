#[cfg(feature = "client")]
mod client;
#[cfg(feature = "sp")]
mod sp;
#[cfg(feature = "enclave")]
mod enclave;
mod error;

#[cfg(feature = "client")]
pub use client::*;
#[cfg(feature = "sp")]
pub use sp::*;
#[cfg(feature = "enclave")]
pub use enclave::*;
pub use error::*;

#[cfg(not(feature = "enclave"))]
const MAX_HOST_SIZE : usize = 256;

//TODO documentation
