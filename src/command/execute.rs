use crate::{command::{CommandError, Commands}, resp::RespValue};

pub fn execute_command(command: Commands, parsed_data: &RespValue) -> Result<RespValue, CommandError>{
    match command {
        Commands::PING => {
            Ok(RespValue::SimpleString(b"PONG".to_vec()))
        },
        Commands::ECHO => {
            match parsed_data {
                RespValue::Arrays(Some(v)) => handle_echo(&v[1]),
                _ => Err(CommandError::InvalidRequest)
            }
        }    
    }
}

fn handle_echo(parsed_data: &RespValue) -> Result<RespValue, CommandError> {
    Ok(parsed_data.clone())
}
