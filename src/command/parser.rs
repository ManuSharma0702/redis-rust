use crate::{command::{CommandError, Commands}, resp::value::RespValue};

impl Commands {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let upper = bytes.iter().map(|b| b.to_ascii_uppercase()).collect::<Vec<u8>>();
        match upper.as_slice() {
            b"PING" => Some(Commands::PING),
            b"ECHO" => Some(Commands::ECHO),
            b"SET" => Some(Commands::SET),
            b"GET" => Some(Commands::GET),
            _ => None
        }
    }
}

pub fn get_command(parsed_input: &RespValue) -> Result<Commands, CommandError> {
    match parsed_input {
        RespValue::Arrays(Some(v)) => {
            match v.first() {
                Some(first) => identify_command(first),
                None => Err(CommandError::UnknownCommand)
            }
        },
        _ => Err(CommandError::InvalidRequest)
    }
}

fn identify_command(input: &RespValue) -> Result<Commands, CommandError> {
    match input {
        RespValue::BulkString(Some(bytes)) => {
            match Commands::from_bytes(bytes) {
                Some(v) => Ok(v),
                None => Err(CommandError::UnknownCommand)
            }
        },
        _ => Err(CommandError::InvalidRequest)
    }
}
