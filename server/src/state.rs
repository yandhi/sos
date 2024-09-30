use std::{collections::HashMap, sync::mpsc::Sender};

pub struct State {
    pub users: HashMap<String, Sender<String>>
}

impl State {
    pub fn new() -> Self {
        Self {
            users: HashMap::new()
        }
    }
}
