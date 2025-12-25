use std::{io::BufReader, net::{TcpListener, TcpStream}} ;

pub fn create_connection(){
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Connection Established");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&stream);
    println!("{buf_reader:?}");
}
