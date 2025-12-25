use crate::{command::{CommandError, Commands}, resp::parse_dispatcher, resp::value::RespValue};


pub fn get_command(input: &[u8]) -> Result<Commands, CommandError> {
    let parsed_input = parse_dispatcher(input).map_err(|_| {
        CommandError::ParseFailed
    })?.result;

    match parsed_input {
        RespValue::Arrays(v) => {
            let input_arr = v.unwrap();
            identify_command(&input_arr[0])
        },
        _ => Err(CommandError::InvalidRequest)
    }
}

fn identify_command(input: &RespValue) -> Result<Commands, CommandError> {
    match input {
        RespValue::BulkString(Some(bytes)) => {
            if bytes == b"PING" {
                Ok(Commands::PING)
            } else {
                Err(CommandError::UnknownCommand)
            }
        },
        _ => Err(CommandError::InvalidRequest)
    }
}
