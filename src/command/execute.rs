use crate::{command::{CommandError, Commands}, resp::RespValue, store::value::Store};

pub fn execute_command(command: Commands, parsed_data: &RespValue, store: &mut Store) -> Result<RespValue, CommandError>{

    match command {
        Commands::PING => {
            Ok(RespValue::SimpleString(b"PONG".to_vec()))
        },
        Commands::ECHO => {
            match parsed_data {
                RespValue::Arrays(Some(v)) => handle_echo(&v[1]),
                _ => Err(CommandError::InvalidRequest)
            }
        },
        Commands::SET => {
            match parsed_data {
                RespValue::Arrays(Some(v)) => handle_set(&v[1..], store),
                _ => Err(CommandError::InvalidRequest)
            }
        },
        Commands::GET => {
            match parsed_data {
                RespValue::Arrays(Some(v)) => handle_get(&v[1..], store),
                _ => Err(CommandError::InvalidRequest)
            }
        }
    }
}

fn handle_echo(parsed_data: &RespValue) -> Result<RespValue, CommandError> {
    Ok(parsed_data.clone())
}

fn handle_set(parsed_data: &[RespValue], store: &mut Store) -> Result<RespValue, CommandError>{
    let key = &parsed_data[0];
    let value = &parsed_data[1];

    match store.set(key, value) {
        Ok(n) => Ok(n),
        Err(_) => Err(CommandError::InvalidRequest)
    }
}

fn handle_get(parsed_data: &[RespValue], store: &mut Store) -> Result<RespValue, CommandError> {
    let key = &parsed_data[0];
    match store.get(key) {
        Ok(n) => Ok(n),
        Err(_) => Err(CommandError::InvalidRequest)
    }
}
