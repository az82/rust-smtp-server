extern crate threadpool;

use std::io::{BufReader};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

mod smtp;


const BIND_ADDRESS :&str = "127.0.0.1";
const BIND_PORT : u16 = 2525;



fn handle_connection(stream: TcpStream) {
    let mut reader = BufReader::new(stream);

    let mut line = String::new();
    loop {
        reader.read_line(&mut line).unwrap();




        println!("{}",line);
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
