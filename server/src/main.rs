use std::{error::Error, io::{Read, Write}, net::SocketAddr, sync::{Arc, Mutex}};

use peer::Peer;
use state::State;
use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

pub mod state;
pub mod peer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:3366").await?;

    let state = Arc::new(Mutex::new(State::new()));

    loop {

        let state = Arc::clone(&state);

        match listener.accept().await {
            Ok((mut socket, addr)) => {
                tokio::spawn(async move {
                    if let Err(e) = accept(state, &mut socket, addr).await  {
                        println!("An error has occurred {}.", e);
                    }
                });
            }
            Err(e) => println!("An error has occured: {}", e)
        }
    }

    Ok(())
}

async fn accept(state: Arc<Mutex<State>>, socket: &mut TcpStream, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    // ===============
    // init sequence.
    // =============== 

    // ask for a username and retrieve it.
    if let Err(e) = socket.try_write(b"Please enter a username.") {
        println!("An error has occurred {}.", e);
    }

    let mut name = String::new();

    let name = match socket.read_to_string(&mut name).await {
        Ok(_) => {
            name
        }   
        Err(e) => {
            println!("An error has occured {}. Disconnecting. ", e);
            return Ok(())
        }
    };

    // make the peer profile that the server interacts with.
    let peer = Peer::new(state.clone(), addr, name).await;

    // tell the server we made it.
    state.lock().unwrap().send_public_message(&format!("{} has connected!", peer.name), &peer);

    // =============
    // loop sequence
    // =============
    loop {
        
        let mut buffer = [0; 1024];
        socket.read(&mut buffer).await.expect("An error has occurred while reading socket.");
        let response = String::from_utf8_lossy(&buffer[..]);
        state.lock().unwrap().send_public_message(&response.into_owned(), &peer);
    }

    Ok(())
}

