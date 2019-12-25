use std::collections::HashMap;

#[derive(Default)]
pub struct Transaction {
    read_cache: HashMap<String, Vec<u8>>,
    write_cache: HashMap<String, Vec<u8>>,
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            read_cache: HashMap::new(),
            write_cache: HashMap::new(),
        }
    }
}
