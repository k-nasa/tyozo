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
}
