use std::{error::Error, io::{self, Read}, net::TcpStream};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3366").expect("Could not connect.");


    loop {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).expect("can't read the stream.");

        println!("{}", String::from_utf8_lossy(&buffer[..]));
    }
}
