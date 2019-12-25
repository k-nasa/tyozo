use std::collections::HashMap;
use std::sync::Mutex;

use crate::command::Command;
use crate::locks::Locks;

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

    // FIXME
    // 基本的にMemdb::exec_commandと同じなのでTraitとしてまとめられるのではないかと思っている
    pub fn exec_command(
        &mut self,
        command: Command,
        locks: &Mutex<Locks>,
    ) -> Result<String, String> {
        match command {
            Command::Set { key, value } => {
                // FIXME 共通処理
                if let None = self.get(&key) {
                    locks.lock().unwrap().write_lock(&key);
                }

                self.set(key, value);
                Ok(String::from("OK"))
            }

            Command::Get { key } => {
                // FIXME 共通処理
                if let None = self.get(&key) {
                    locks.lock().unwrap().read_lock(&key);
                }

                match self.get(key) {
                    None => Ok(String::from("None")),
                    Some(v) => Ok(String::from_utf8(v).unwrap()),
                }
            }
            Command::Del { keys } => {
                keys.iter().for_each(|key| {
                    // FIXME 共通処理
                    if let None = self.get(&key) {
                        locks.lock().unwrap().read_lock(&key);
                    }
                });

                let result = self.del(keys);
                Ok(format!("{}", result))
            }
            Command::Exec => todo!(),
            Command::Abort => todo!(),
            _ => Err(String::from("ERR unsupport transaction command")),
        }
    }

    fn set(&mut self, key: impl AsRef<str>, value: impl AsRef<[u8]>) {
        self.write_cache
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
    }

    fn get(&mut self, key: impl AsRef<str>) -> Option<Vec<u8>> {
        if let Some(v) = self.read_cache.get(&key.as_ref().to_owned()) {
            return Some(v.to_vec());
        }

        if let Some(v) = self.write_cache.get(&key.as_ref().to_owned()) {
            return Some(v.to_vec());
        }

        None
    }

    fn del(&mut self, keys: Vec<impl AsRef<str>>) -> usize {
        keys.into_iter()
            .map(|key| self.write_cache.remove(&key.as_ref().to_owned()))
            .filter(|v| v.is_some())
            .count()
    }
}
