mod lexer;
mod parser;

use std::collections::HashMap;

fn main() -> Result<(), String> {
    tyozo("hoge")
}

fn tyozo<S: Into<String>>(input: S) -> Result<(), String> {
    let command = parser::parse(input)?;

    // TODO exec command

    // TODO return exec result

    Ok(())
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

    pub fn set(&mut self, key: impl AsRef<str>, value: impl AsRef<[u8]>) {
        self.inner
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
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
