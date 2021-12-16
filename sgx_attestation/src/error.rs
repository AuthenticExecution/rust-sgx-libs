#[derive(Debug)]
pub enum Error {
    BindPortError,
    SigstructDeserializationError,
    SpConfigDeserializationError,
    EnclaveConnectionError,
    SpConnectionError,
    SpConfigNotPresent,
    SigstructNotPresent,
    MasterKeyNotPresent,
    EnclaveHostTooLong,
    FailedToInitEnclaveRaContext,
    #[cfg(feature = "enclave")]
    AttestationFailed(anyhow::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
        -> Result<(), std::fmt::Error> {
            write!(f, "{:?}", self)
        }
}

impl std::error::Error for Error {}
