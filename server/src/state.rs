use std::collections::HashMap;

use rand::{thread_rng, Rng};
use tokio::sync::mpsc::UnboundedSender;

pub struct State {
   pub peers: HashMap<usize, UnboundedSender<String>>,
}

impl State {

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

}