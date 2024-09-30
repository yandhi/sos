use std::{io::{BufRead, BufReader, Read, Write}, net::{TcpListener, TcpStream}, sync::{mpsc::channel, Arc, Mutex}, thread::spawn};

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

    let mut write_stream = stream.try_clone().expect("could not clone connection");

    write_stream.write_all(b"Welcome! Please enter a username.").expect("Failed to ask for username.");

    let mut reader = BufReader::new(&stream);

    let mut username = String::new();

    reader.read_line(&mut username).expect("failed to read username from client");

    let (tx, rx) = channel::<String>();

    state.lock().unwrap().users.insert(username.to_string(), tx);    

    loop {
        // read input from stream.

        let mut buffer = String::new();

        reader.read_line(&mut buffer).expect("failed to read into buffer");

        if buffer.contains("ping") {
          write_stream.write_all(b"Hello!");  
        }

        println!("[{}] {}", username.trim(), buffer);

    }
}

