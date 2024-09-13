use std::collections::HashMap;

use rand::{thread_rng, Rng};
use tokio::sync::mpsc::UnboundedSender;

use crate::peer::Peer;

pub struct State {
   pub peers: HashMap<usize, UnboundedSender<String>>,
}

impl State {
    pub fn new() -> State {
        Self {
            peers: HashMap::new()
        }
    }
    pub fn gen_new_peer_id(&self) -> usize {
        let mut peer_id = self.gen_new_peer_id();

        if self.peers.contains_key(&peer_id) { 
            // this shouldn't be a hold-up but if it is, a thread can be made for it eventually.
            while !self.peers.contains_key(&peer_id)  {
                peer_id = self.gen_new_peer_id();
            }
        }
        
        peer_id
    }

    fn gen_peer_id() -> usize {
        thread_rng().gen_range(0.0..=1E5) as usize
    }

    // Send a message from a peer to all.
    pub fn send_public_message(&mut self, msg: &String, sender: &Peer) {
        for(id, tx) in &mut self.peers {
            if id != &sender.id {
                match tx.send(msg.clone()) {
                    Ok(_) => {}
                    Err(e) => println!("An error has occurred {}", e),
                }            
            }
        }
    }

    // Send a message to all clients.
    pub fn broadcast(&mut self, msg: &String) {
        for (id, tx) in &mut self.peers {
            match tx.send(msg.clone()) {
                Ok(_) => {}
                Err(e) => println!("An error has occurred {}", e),
            }
        }
    }

}