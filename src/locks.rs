use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::{Arc, Mutex};

pub struct Locks {
    inner: Arc<Mutex<HashMap<String, RwLock<Vec<u8>>>>>,
}

impl Locks {
    pub fn new() -> Locks {
        Locks {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
