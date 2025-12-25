use redis_rust::server;

fn main() {
    println!("Starting server on 127.0.0.1:6379");
    server::tcp::create_connection();
}
