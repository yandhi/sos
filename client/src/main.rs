use std::{io::{stdin, Read, Write}, net::TcpStream, thread::spawn};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3366").expect("Could not connect.");
    
    let mut input_stream = stream.try_clone().expect("couldn't clone the stream.");

    // Input Thread.
    spawn(move || loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("could not read input stream.");
        input_stream.write_all(buffer.as_bytes()).expect("could not transmit input over channel.");
    });

    
    // Output loop
    loop {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("could not read the stream");
        
        println!("{}", String::from_utf8_lossy(&buffer));
    }
}
