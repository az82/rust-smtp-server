extern crate threadpool;

use std::io::{BufReader};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

mod smtp;


const BIND_ADDRESS :&str = "127.0.0.1";
const BIND_PORT : u16 = 2525;



fn handle_connection(stream: TcpStream) {
    let mut parser = smtp::Parser::new();

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    loop {
        reader.read_line(&mut line).unwrap();
        match parser.feed_line(&line) { // TODO: Send Response codes
            Ok(_) => {}, // TODO: Send 250 OK
            Err(e) => { break } // TODO
        }
    }

    println!("Sender domain: {}", parser.get_sender_domain().unwrap());

    for message in parser.get_messages().unwrap() {
        println!("From: {}", message.get_sender());
        println!("To: {}", message.get_recipients().join(", "));
        println!("\n{}\n", message.get_data());
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
