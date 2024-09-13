use tokio::{io, net::TcpListener};

pub mod state;
pub mod peer;

#[tokio::main]
async fn main() -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:3366").await?;

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                // handle each connection here.
            }
            Err(e) => println!("An error has occured: {}", e)
        }
    }

    Ok(())
}
