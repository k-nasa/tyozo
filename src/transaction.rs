use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

use crate::command::Command;
use crate::locks::Locks;
use crate::memdb::Memdb;

#[derive(Default, Debug)]
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
        memdb: &RwLock<Memdb>,
    ) -> Result<String, String> {
        match command {
            Command::Set { key, value } => {
                // FIXME 共通処理
                if self.get(&key).is_none() {
                    locks.lock().unwrap().write_lock(&key);
                }

                self.write_set(key, value);
                Ok(String::from("OK"))
            }

            Command::Get { key } => match self.get(&key) {
                None => match memdb.read().unwrap().get(&key) {
                    None => Ok("None".to_owned()),
                    Some(v) => {
                        locks.lock().unwrap().read_lock(&key);

                        self.read_set(key, v.clone());
                        Ok(String::from_utf8(v).unwrap())
                    }
                },
                Some(v) => {
                    self.read_set(key, v.clone());
                    Ok(String::from_utf8(v).unwrap())
                }
            },
            Command::Del { keys } => {
                keys.iter().for_each(|key| {
                    // FIXME 共通処理
                    if self.get(&key).is_none() {
                        locks.lock().unwrap().read_lock(&key);
                    }
                });

                let result = self.del(keys);
                Ok(format!("{}", result))
            }
            Command::Exec => {
                self.read_cache.keys().for_each(|k| {
                    locks.lock().unwrap().read_unlock(k);
                });

                let mut db = memdb.write().unwrap();

                self.write_cache.iter().for_each(|(k, v)| {
                    db.set(k, v);
                    locks.lock().unwrap().write_unlock(k);
                });

                self.read_cache = HashMap::new();
                self.write_cache = HashMap::new();

                Ok(String::from("OK"))
            }
            Command::Abort => {
                self.clear_lock(locks);

                Ok(String::from("Abort transaction"))
            }
            _ => Err(String::from("ERR unsupport transaction command")),
        }
    }

    fn read_set(&mut self, key: impl AsRef<str>, value: impl AsRef<[u8]>) {
        self.read_cache
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
    }

    fn write_set(&mut self, key: impl AsRef<str>, value: impl AsRef<[u8]>) {
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

    pub fn clear_lock(&mut self, locks: &Mutex<Locks>) {
        self.read_cache.keys().for_each(|k| {
            locks.lock().unwrap().read_unlock(k);
        });

        self.write_cache.keys().for_each(|k| {
            locks.lock().unwrap().write_unlock(k);
        });

        self.read_cache = HashMap::new();
        self.write_cache = HashMap::new();
    }
}
