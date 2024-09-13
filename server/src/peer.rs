use tokio::{net::unix::SocketAddr, sync::mpsc::{unbounded_channel, UnboundedReceiver}};

use crate::state::State;

struct Peer {
    pub id: usize,
    pub addr: SocketAddr,
    pub rx: UnboundedReceiver<String>
}

impl Peer {
    pub fn new(mut state: State, addr: SocketAddr) -> Self {
        let (tx, rx) = unbounded_channel::<String>();

        

        // generate a random id for the peer.
        // this works a lot better than the socketaddr for some reason.
        let id = state.gen_new_peer_id();

        state.peers.insert(id.clone(), tx);

        Self {
            id,
            addr,
            rx
        }
    }
}
