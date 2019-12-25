use std::collections::HashMap;

use crate::command::Command;

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

    pub fn exec_command(&mut self, command: Command) -> Result<String, String> {
        match command {
            Command::Set { key, value } => todo!(),
            Command::SetNX { key, value } => todo!(),
            Command::Get { key } => todo!(),
            Command::Del { keys } => todo!(),
            Command::Exec => todo!(),
            Command::Abort => todo!(),
            _ => Err(String::from("ERR unsupport transaction command")),
        }
    }
}
