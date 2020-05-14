#[derive(Copy, Clone, Debug)]
pub enum ResultCode {
    Ok,
    IllegalCommand,
    IllegalPayload,
    InternalError,
    BadRequest,
    CryptoError,
    GenericError
}

impl ResultCode {
    pub fn from_u8(value : u8) -> ResultCode {
        match value {
            0 => ResultCode::Ok,
            1 => ResultCode::IllegalCommand,
            2 => ResultCode::IllegalPayload,
            3 => ResultCode::InternalError,
            4 => ResultCode::BadRequest,
            5 => ResultCode::CryptoError,
            _ => ResultCode::GenericError
        }
    }
}

impl std::fmt::Display for ResultCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
        -> Result<(), std::fmt::Error> {
            write!(f, "{:?}", self)
        }
}

#[derive(Debug)]
pub struct ResultMessage {
    code : ResultCode,
    payload : Option<Vec<u8>>
}

impl ResultMessage {
    pub fn new(code : ResultCode, payload : Option<Vec<u8>>) -> ResultMessage {
        ResultMessage {
            code,
            payload
        }
    }

    pub fn get_code(&self) -> &ResultCode {
        &self.code
    }

    pub fn get_code_u8(&self) -> u8 {
        self.code as u8
    }

    pub fn get_payload(&self) -> Option<&[u8]> {
        match &self.payload {
            Some(p) => Some(&p),
            None    => None
        }
    }

    pub fn payload_as_string(&self) -> String {
        match &self.payload {
            Some(p) => format!("{}", std::str::from_utf8(p).unwrap_or(&format!("{:?}", p))),
            None    => String::from("<No data>")
        }
    }
}
