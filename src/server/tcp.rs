use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread} ;

use crate::{command::{execute_command, get_command, CommandError}, 
    resp::{parse_dispatcher, serializer::serializer, ParseError, RespValue}, server::value::ServerError, store::value::Store};

impl From<CommandError> for ServerError {
    fn from(e: CommandError) -> Self{
        ServerError::Command(e)
    }
}

impl From<ParseError> for ServerError {
    fn from(e: ParseError) -> Self {
        ServerError::Parse(e)
    }
}

pub fn create_connection(){
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let store = Arc::new(Mutex::new(Store::new()));
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        let store = store.clone();
        thread::spawn(move||{
            handle_connection(stream, store);
        });
    }
}

fn handle_connection(mut stream: TcpStream, store: Arc<Mutex<Store>>) {
    let mut buf = [0u8; 4096];

    loop {
        let n = match stream.read(&mut buf) {
            Ok(0) => break, // client closed connection
            Ok(n) => n,
            Err(_) => break,
        };

        let data = &buf[..n];

        let output_data = {
            let mut store = store.lock().unwrap();
            process(data, &mut store).unwrap_or_else(|error| {
                let res = error_to_resp(error);
                serializer(&res).unwrap()
            })
        };

        if stream.write_all(&output_data).is_err() {
            break;
        }
    }
}


fn process(data: &[u8], store: &mut Store) -> Result<Vec<u8>, ServerError>{

    let parsed_data = parse_dispatcher(data)?.result;
    let command = get_command(&parsed_data)?;
    let result = execute_command(command, &parsed_data, store)?;

    let output_data = serializer(&result)?;
    Ok(output_data)
}

fn error_to_resp(error: ServerError) -> RespValue {
    match error {
        ServerError::Command(CommandError::UnknownCommand) => RespValue::Error(b"ERR unknown command".to_vec()),
        ServerError::Parse(_) => RespValue::Error(b"ERR protocol error".to_vec()),
        ServerError::Command(CommandError::ParseFailed) => RespValue::Error(b"ERR protocol error".to_vec()),
        ServerError::Command(CommandError::InvalidRequest) => RespValue::Error(b"ERR unknown command".to_vec())
    }
}
