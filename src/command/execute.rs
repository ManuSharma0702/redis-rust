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
    if parsed_data.len() < 2 {
        return Err(CommandError::InvalidRequest);
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::RespValue;
    use crate::command::Commands;
    use crate::store::value::Store;

    fn bulk(v: &str) -> RespValue {
        RespValue::BulkString(Some(v.as_bytes().to_vec()))
    }

    fn array(v: Vec<RespValue>) -> RespValue {
        RespValue::Arrays(Some(v))
    }

    #[test]
    fn ping_returns_pong() {
        let mut store = Store::new();
        let result = execute_command(
            Commands::PING,
            &RespValue::SimpleString(vec![]),
            &mut store,
        )
        .unwrap();

        assert_eq!(result, RespValue::SimpleString(b"PONG".to_vec()));
    }

    #[test]
    fn echo_returns_same_value() {
        let mut store = Store::new();

        let input = array(vec![
            RespValue::SimpleString(b"ECHO".to_vec()),
            bulk("hello"),
        ]);

        let result = execute_command(Commands::ECHO, &input, &mut store).unwrap();
        assert_eq!(result, bulk("hello"));
    }

    #[test]
    fn set_then_get_returns_value() {
        let mut store = Store::new();

        let set_cmd = array(vec![
            RespValue::SimpleString(b"SET".to_vec()),
            bulk("key"),
            bulk("value"),
        ]);

        let set_res = execute_command(Commands::SET, &set_cmd, &mut store).unwrap();
        assert_eq!(set_res, RespValue::SimpleString(b"OK".to_vec()));

        let get_cmd = array(vec![
            RespValue::SimpleString(b"GET".to_vec()),
            bulk("key"),
        ]);

        let get_res = execute_command(Commands::GET, &get_cmd, &mut store).unwrap();
        assert_eq!(get_res, bulk("value"));
    }

    #[test]
    fn get_non_existing_key_returns_null() {
        let mut store = Store::new();

        let get_cmd = array(vec![
            RespValue::SimpleString(b"GET".to_vec()),
            bulk("missing"),
        ]);

        let result = execute_command(Commands::GET, &get_cmd, &mut store).unwrap();
        assert_eq!(result, RespValue::BulkString(None));
    }

    #[test]
    fn set_with_invalid_args_fails() {
        let mut store = Store::new();

        let bad_set = array(vec![
            RespValue::SimpleString(b"SET".to_vec()),
            bulk("only_key"),
        ]);

        let result = execute_command(Commands::SET, &bad_set, &mut store);
        assert!(result.is_err());
    }
}








