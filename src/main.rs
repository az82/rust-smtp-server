extern crate threadpool;

use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

mod smtp;


const BIND_ADDRESS: &str = "127.0.0.1";
const BIND_PORT: u16 = 2525;


fn handle_connection(mut stream: TcpStream) {
    let mut parser = smtp::Parser::new();

    let mut reader = BufReader::new(stream.try_clone().unwrap());

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        // read_line will leave trailing newlines which must be removed
        match parser.feed_line(line.trim_right_matches(|c: char|{ c== '\n' || c == '\r'})) {
            Ok("") => {}
            Ok("221 Bye") => { break; }
            Ok(s) => {
                stream.write(s.as_bytes());
                stream.write("\n".as_bytes());
            },
            Err(e) => {
                stream.write(e.as_bytes());
                stream.write("\n".as_bytes());
                // Close connection on error
                return;
            }
        }
    }

    for message in parser.get_messages().unwrap() {
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
