use std::collections::HashMap;

mod lexer;
mod parser;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Command {
    Set { key: String, value: String },
    SetNX { key: String, value: String },
    Get { key: String },
    Del { keys: Vec<String> },
}

type MemdbInner = HashMap<String, Vec<u8>>;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Memdb {
    inner: MemdbInner,
}

impl Memdb {
    pub fn new() -> Memdb {
        Memdb {
            inner: HashMap::new(),
        }
    }

    pub fn exec<S: Into<String>>(&mut self, input: S) -> Result<String, String> {
        let command = parser::parse(input)?;

        self.exec_command(command)
    }

    pub fn exec_command(&mut self, command: Command) -> Result<String, String> {
        match command {
            Command::Set { key, value } => {
                self.set(key, value);
                Ok(String::from("OK"))
            }
            Command::SetNX { key, value } => {
                self.setnx(key, value)?;
                Ok(String::from("OK"))
            }
            Command::Get { key } => match self.get(key) {
                None => Ok(String::from("None")),
                Some(v) => Ok(String::from_utf8(v).unwrap()),
            },
            Command::Del { keys } => {
                let result = self.del(keys);
                Ok(format!("{}", result))
            }
        }
    }

    pub fn set(&mut self, key: impl AsRef<str>, value: impl AsRef<[u8]>) {
        self.inner
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
    }

    pub fn setnx(&mut self, key: impl AsRef<str>, value: impl AsRef<[u8]>) -> Result<(), String> {
        if self.get(&key).is_some() {
            return Err(String::from("ERR key is already exists"));
        }

        self.inner
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());

        Ok(())
    }

    pub fn get(&mut self, key: impl AsRef<str>) -> Option<Vec<u8>> {
        self.inner.get(&key.as_ref().to_owned()).cloned()
    }

    pub fn del(&mut self, keys: Vec<impl AsRef<str>>) -> usize {
        keys.into_iter()
            .map(|key| self.inner.remove(&key.as_ref().to_owned()))
            .filter(|v| v.is_some())
            .count()
    }

    pub fn inner(&self) -> &MemdbInner {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memdb_set() {
        let mut memdb = Memdb::new();

        memdb.set("key", "value");
        assert_eq!(memdb.inner().get("key"), Some(&b"value".to_vec()));

        // value is override
        memdb.set("key", "next value");
        assert_eq!(memdb.inner().get("key"), Some(&b"next value".to_vec()));
    }

    #[test]
    fn test_memdb_setnx() {
        let mut memdb = Memdb::new();

        let result = memdb.setnx("key", "value");
        assert!(result.is_ok());
        assert_eq!(memdb.inner().get("key"), Some(&b"value".to_vec()));

        // value is not override
        let result = memdb.setnx("key", "next value");
        assert!(result.is_err());
        assert_eq!(memdb.inner().get("key"), Some(&b"value".to_vec()));
    }

    #[test]
    fn test_memdb_get() {
        let mut memdb = Memdb::new();

        memdb.set("key", "value");

        assert_eq!(memdb.get("key"), Some(b"value".to_vec()));

        assert_eq!(memdb.get("not setted key"), None);
    }

    #[test]
    fn test_memdb_delete() {
        let mut memdb = Memdb::new();
        memdb.set("key", "value");

        let delete_count = memdb.del(vec!["key", "key2"]);

        assert_eq!(delete_count, 1);
        assert_eq!(memdb.get("key"), None);

        let delete_count = memdb.del(vec!["key"]);
        assert_eq!(delete_count, 0);
    }
}
