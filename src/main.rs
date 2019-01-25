extern crate threadpool;

use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

mod smtp;

const BIND_ADDRESS: &str = "127.0.0.1";
const BIND_PORT: u16 = 2525;

fn handle_connection(stream: TcpStream) {
    let result = smtp::handle_connection(stream).unwrap();

    println!("Sender domain: {}", result.get_sender_domain().unwrap());
    for message in result.get_messages().unwrap() {
        println!("Message from: {}", message.get_sender());
        println!("To: {}", message.get_recipients().join(", "));
        println!("{}", message.get_data());
    }
}

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", BIND_ADDRESS, BIND_PORT)).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
