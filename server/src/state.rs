use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};

pub struct State {
    pub users: HashMap<String, Arc<Mutex<TcpStream>>>
}

impl State {
    pub fn new() -> Self {
        Self {
            users: HashMap::new()
        }
    }
}
