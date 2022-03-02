use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0;2048];
    stream.read(&mut buffer).unwrap();

    println!("Response from server: {:?}", str::from_utf8(&mut buffer).unwrap());
}
