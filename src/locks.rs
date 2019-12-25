use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Locks {
    // NOTE sync::RwLock で出来そうだが
    // ライフタイムの関係で今の自分の実力ではよく分からなかったので
    // RWLock enumで管理するようにしている。
    // TODO @k-nasa コレで良いのか判断してくれ！ deadline: 2020/2/31
    hashmap: Arc<Mutex<HashMap<String, RWLock>>>,
}

#[derive(Debug, PartialOrd, PartialEq)]
enum RWLock {
    Read(usize),
    Write,
}

impl Locks {
    pub fn new() -> Locks {
        Locks {
            hashmap: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn read_lock(&mut self, key: &str) {
        // FIXME Write lock の開放をループで待って良いのか？という気持ち
        loop {
            let mut hashmap = self.hashmap.lock().unwrap();

            if let Some(lock) = hashmap.get_mut(key) {
                match lock {
                    RWLock::Read(count) => *count += 1,
                    RWLock::Write => continue,
                };
                break;
            } else {
                hashmap.insert(key.to_owned(), RWLock::Read(1));
                break;
            }
        }
    }

    pub fn read_unlock(&mut self, key: &str) {
        // TODO refactor this method

        let mut hashmap = self.hashmap.lock().unwrap();

        if let Some(lock) = hashmap.get_mut(key.clone()) {
            match lock {
                RWLock::Read(count) => {
                    *count -= 1;
                    if *count == 0 {
                        hashmap.remove(key);
                    }
                }
                RWLock::Write => panic!("Attempting to release write lock"),
            };
        } else {
            panic!("not found read lock")
        }
    }

    pub fn write_unlock(&mut self, key: &str) {
        // TODO refactor this method

        let mut hashmap = self.hashmap.lock().unwrap();

        if let Some(lock) = hashmap.get_mut(key.clone()) {
            match lock {
                RWLock::Write => hashmap.remove(key),
                RWLock::Read(_) => panic!("Attempting to release read lock"),
            };
        } else {
            panic!("not found write lock")
        }
    }

    pub fn write_lock(&mut self, key: &str) {
        // FIXME read lock の開放をループで待って良いのか？という気持ち
        loop {
            let mut hashmap = self.hashmap.lock().unwrap();

            if let Some(lock) = hashmap.get_mut(key.clone()) {
                match lock {
                    RWLock::Read(0) => {
                        hashmap.insert(key.to_owned(), RWLock::Write);
                        break;
                    }
                    _ => continue,
                };
            } else {
                hashmap.insert(key.to_owned(), RWLock::Write);
                break;
            }
        }
    }
}

#[test]
fn test_read_lock() {
    let mut locks = Locks::new();

    let key = "key";

    locks.read_lock(key);
    locks.read_lock(key);
    locks.read_lock(key);
    locks.read_lock(key);

    let hashmap = locks.hashmap.lock().unwrap();
    let lock = hashmap.get(key).unwrap();

    assert_eq!(lock, &RWLock::Read(4));
}

#[test]
fn test_read_unlock() {
    let mut locks = Locks::new();

    let key = "key";

    locks.read_lock(key);
    locks.read_unlock(key);

    let hashmap = locks.hashmap.lock().unwrap();
    assert!(hashmap.get(key).is_none());
}

#[test]
fn test_write_lock() {
    let mut locks = Locks::new();

    let key = "key";

    locks.write_lock(key);

    let hashmap = locks.hashmap.lock().unwrap();
    let lock = hashmap.get(key).unwrap();

    assert_eq!(lock, &RWLock::Write);
}

#[test]
fn test_write_unlock() {
    let mut locks = Locks::new();

    let key = "key";

    locks.write_lock(key);
    locks.write_unlock(key);

    let hashmap = locks.hashmap.lock().unwrap();
    assert!(hashmap.get(key).is_none());
}

#[test]
#[should_panic]
fn test_write_unlock_panic_when_not_found_key() {
    let mut locks = Locks::new();

    locks.write_unlock("not found key");
}

#[test]
#[should_panic]
fn test_write_unlock_panic_when_read_lock() {
    let mut locks = Locks::new();

    locks.read_lock("key");
    locks.write_unlock("key");
}

#[test]
#[should_panic]
fn test_read_unlock_panic_when_not_found_key() {
    let mut locks = Locks::new();

    locks.read_unlock("not found key");
}

#[test]
#[should_panic]
fn test_write_unlock_panic_when_write_lock() {
    let mut locks = Locks::new();

    locks.write_lock("key");
    locks.read_unlock("key");
}
