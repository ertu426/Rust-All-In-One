use std::net::TcpListener;
use std::io::{Read, Write};
use std::str;

fn main() {
    let listenner = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Start The Server in Port 3000 ...");

    for stream in listenner.incoming() {
        let mut stream = stream.unwrap();
        println!("stream in connect");
        let mut buffer = [0; 2048];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
        println!("{:?}", str::from_utf8(&mut buffer));
    }
}
