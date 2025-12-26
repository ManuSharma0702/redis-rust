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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::value::RespValue;

    fn bulk(s: &str) -> RespValue {
        RespValue::BulkString(Some(s.as_bytes().to_vec()))
    }

    fn array(v: Vec<RespValue>) -> RespValue {
        RespValue::Arrays(Some(v))
    }

    #[test]
    fn from_bytes_ping_case_insensitive() {
        assert_eq!(Commands::from_bytes(b"PING"), Some(Commands::PING));
        assert_eq!(Commands::from_bytes(b"ping"), Some(Commands::PING));
        assert_eq!(Commands::from_bytes(b"PiNg"), Some(Commands::PING));
    }

    #[test]
    fn from_bytes_unknown_command() {
        assert_eq!(Commands::from_bytes(b"FOO"), None);
    }

    #[test]
    fn get_command_ping() {
        let input = array(vec![bulk("PING")]);
        let cmd = get_command(&input).unwrap();
        assert_eq!(cmd, Commands::PING);
    }

    #[test]
    fn get_command_echo() {
        let input = array(vec![bulk("ECHO"), bulk("hello")]);
        let cmd = get_command(&input).unwrap();
        assert_eq!(cmd, Commands::ECHO);
    }

    #[test]
    fn get_command_set() {
        let input = array(vec![bulk("SET"), bulk("key"), bulk("value")]);
        let cmd = get_command(&input).unwrap();
        assert_eq!(cmd, Commands::SET);
    }

    #[test]
    fn get_command_get() {
        let input = array(vec![bulk("GET"), bulk("key")]);
        let cmd = get_command(&input).unwrap();
        assert_eq!(cmd, Commands::GET);
    }

    #[test]
    fn get_command_unknown_command() {
        let input = array(vec![bulk("UNKNOWN")]);
        let err = get_command(&input).unwrap_err();
        assert_eq!(err, CommandError::UnknownCommand);
    }

    #[test]
    fn get_command_empty_array() {
        let input = RespValue::Arrays(Some(vec![]));
        let err = get_command(&input).unwrap_err();
        assert_eq!(err, CommandError::UnknownCommand);
    }

    #[test]
    fn get_command_non_array_input() {
        let input = bulk("PING");
        let err = get_command(&input).unwrap_err();
        assert_eq!(err, CommandError::InvalidRequest);
    }

    #[test]
    fn get_command_non_bulkstring_command() {
        let input = array(vec![RespValue::Integer(1)]);
        let err = get_command(&input).unwrap_err();
        assert_eq!(err, CommandError::InvalidRequest);
    }
}

