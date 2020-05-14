#[derive(Copy, Clone, Debug)]
pub enum CommandCode {
    AddConnection,
    CallEntrypoint,
    RemoteOutput,
    LoadSM,
    Ping,
    ModuleOutput
}

impl CommandCode {
    pub fn from_u16(value : u16) -> Option<CommandCode> {
        match value {
            0 => Some(CommandCode::AddConnection),
            1 => Some(CommandCode::CallEntrypoint),
            2 => Some(CommandCode::RemoteOutput),
            3 => Some(CommandCode::LoadSM),
            4 => Some(CommandCode::Ping),
            5 => Some(CommandCode::ModuleOutput),
            _ => None
        }
    }
}

impl std::fmt::Display for CommandCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
        -> Result<(), std::fmt::Error> {
            write!(f, "{:?}", self)
        }
}

#[derive(Debug)]
pub struct CommandMessage {
    code : CommandCode,
    payload : Option<Vec<u8>>
}

impl CommandMessage {
    pub fn new(code : CommandCode, payload : Option<Vec<u8>>) -> CommandMessage {
        CommandMessage {
            code,
            payload
        }
    }

    pub fn get_code(&self) -> &CommandCode {
        &self.code
    }

    pub fn get_code_u16(&self) -> u16 {
        self.code as u16
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
