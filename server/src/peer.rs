use std::{net::SocketAddr, sync::{Arc, Mutex}};

use tokio::{sync::mpsc::{unbounded_channel, UnboundedReceiver}};

use crate::state::State;

pub struct Peer {
    pub id: usize,
    pub addr: SocketAddr,
    pub name: String,
    pub rx: UnboundedReceiver<String>
}

impl Peer {
    pub async fn new(mut state: Arc<Mutex<State>>, addr: SocketAddr, name: String) -> Self {
        let (tx, rx) = unbounded_channel::<String>();

        let mut state = state.lock().unwrap();

        // generate a random id for the peer.
        // this works a lot better than the socketaddr for some reason.
        let id = &state.gen_new_peer_id();

        state.peers.insert(id.clone(), tx);

        Self {
            id: id.clone(),
            addr,
            name,
            rx
        }
    }
}
