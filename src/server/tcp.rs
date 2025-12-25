use std::{io::{Read, Write}, net::{TcpListener, TcpStream}} ;

use crate::{command::{execute_command, get_command, CommandError}, 
    resp::{parse_dispatcher, serializer::serializer, ParseError, RespValue}, server::value::ServerError};

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

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Connection Established");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0u8; 512];
    let n = stream.read(&mut buf).unwrap();
    let data = &buf[..n];
    let output_data = process(data).unwrap_or_else(|error| {
        let res = error_to_resp(error);
        serializer(&res).unwrap()
    });
    stream.write_all(&output_data).unwrap();
}

fn process(data: &[u8]) -> Result<Vec<u8>, ServerError>{

    let parsed_data = parse_dispatcher(data)?.result;
    let command = get_command(&parsed_data)?;
    let result = execute_command(command, &parsed_data)?;

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
