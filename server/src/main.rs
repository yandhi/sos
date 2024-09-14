use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread::spawn};

use state::State;

pub mod state;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3366").expect("Failed to bind listener.");

    let state = Arc::new(Mutex::new(State::new()));

    for stream in listener.incoming() {

        let state = Arc::clone(&state);

        match stream {
            Ok(stream) => {
                spawn(|| {
                    handle(state, stream)
                });
            }
            Err(e) => eprintln!("An error has occurred. {}", e)
        }
    }
}

fn handle(state: Arc<Mutex<State>>, mut stream: TcpStream) {
    // init phase

    stream.write(b"Welcome! Please enter a username.").expect("Failed to ask for username.");

    let mut username_buffer = [0; 1024];

    stream.read(&mut username_buffer).expect("failed to read username from client");

    let username = String::from_utf8_lossy(&username_buffer[..]);

    // TODO: username sanitation

    let stream = Arc::new(Mutex::new(stream));

    state.lock().unwrap().users.insert(username.to_string(), Arc::clone(&stream));

    loop {

        let stream = Arc::clone(&stream);
        // read input from stream.

        let mut buffer = [0; 1024];

        stream.lock().unwrap().read(&mut buffer).expect("failed to read into buffer");

        println!("{}", String::from_utf8_lossy(&buffer[..]));
    }
}

