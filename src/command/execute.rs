use crate::{command::{CommandError, Commands}, resp::RespValue};

pub fn execute_command(command: Commands) -> Result<RespValue, CommandError>{
    match command {
        Commands::PING => {
            Ok(RespValue::SimpleString(b"PONG".to_vec()))
        }
    }
}
