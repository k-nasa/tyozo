use std::collections::HashMap;

use crate::command::Command;
use crate::parser;

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

    /// # Example
    /// ```
    /// use tyozo::Memdb;
    /// let mut db = Memdb::new();
    ///
    /// let result = db.exec("set hoge value");
    /// assert_eq!(result, Ok(String::from("OK")));
    ///
    /// let result = db.exec("get hoge");
    /// assert_eq!(result, Ok(String::from("value")));
    ///
    /// db.exec("set fuga value").unwrap();
    /// let result = db.exec("del hoge fuga");
    /// assert_eq!(result, Ok(String::from("2")));
    ///
    /// let result = db.exec("setnx hoge value");
    /// assert!(result.is_ok());
    ///
    /// let result = db.exec("setnx hoge value");
    /// assert!(result.is_err());
    /// ```
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
            Command::Multi => todo!(),
            Command::Exec => todo!(),
            Command::Abort => todo!(),
            _ => unreachable!(),
        }
    }

    /// # Example
    /// ```
    /// use tyozo::Memdb;
    /// let mut memdb = Memdb::new();
    ///
    /// memdb.set("key", "value");
    /// assert_eq!(memdb.inner().get("key"), Some(&b"value".to_vec()));
    ///
    /// // value is override
    /// memdb.set("key", "next value");
    /// assert_eq!(memdb.inner().get("key"), Some(&b"next value".to_vec()));
    /// ```
    pub fn set(&mut self, key: impl AsRef<str>, value: impl AsRef<[u8]>) {
        self.inner
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
    }

    /// # Example
    /// ```
    /// use tyozo::Memdb;
    /// let mut memdb = Memdb::new();

    /// let result = memdb.setnx("key", "value");
    /// assert!(result.is_ok());
    /// assert_eq!(memdb.inner().get("key"), Some(&b"value".to_vec()));

    /// // value is not override
    /// let result = memdb.setnx("key", "next value");
    /// assert!(result.is_err());
    /// assert_eq!(memdb.inner().get("key"), Some(&b"value".to_vec()));
    /// ```
    pub fn setnx(&mut self, key: impl AsRef<str>, value: impl AsRef<[u8]>) -> Result<(), String> {
        if self.get(&key).is_some() {
            return Err(String::from("ERR key is already exists"));
        }

        self.inner
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());

        Ok(())
    }

    /// # Exmaple
    /// ```
    /// use tyozo::Memdb;
    /// let mut memdb = Memdb::new();
    ///
    /// memdb.set("key", "value");
    ///
    /// assert_eq!(memdb.get("key"), Some(b"value".to_vec()));
    ///
    /// assert_eq!(memdb.get("not setted key"), None);
    /// ```
    pub fn get(&mut self, key: impl AsRef<str>) -> Option<Vec<u8>> {
        self.inner.get(&key.as_ref().to_owned()).cloned()
    }

    /// # Exmaple
    /// ```
    /// use tyozo::Memdb;
    /// let mut memdb = Memdb::new();
    /// memdb.set("key", "value");
    ///
    /// let delete_count = memdb.del(vec!["key", "key2"]);
    ///
    /// assert_eq!(delete_count, 1);
    /// assert_eq!(memdb.get("key"), None);
    ///
    /// let delete_count = memdb.del(vec!["key"]);
    /// assert_eq!(delete_count, 0);
    /// ```
    pub fn del(&mut self, keys: Vec<impl AsRef<str>>) -> usize {
        keys.into_iter()
            .map(|key| self.inner.remove(&key.as_ref().to_owned()))
            .filter(|v| v.is_some())
            .count()
    }

    pub fn inner(&self) -> &MemdbInner {
        &self.inner
    }

    /// # Example
    /// ```
    /// use tyozo::Memdb;
    ///
    /// let mut memdb = Memdb::new();
    /// memdb.set("k", "v");
    ///
    /// let serialized = memdb.serialize();
    ///
    /// assert_eq!(serialized, vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 107, 118]);
    /// ```
    pub fn serialize(&self) -> Vec<u8> {
        self.inner.iter().fold(vec![], |mut buf, (key, value)| {
            let key_length_bytes = key.len().to_be_bytes();
            let value_length_bytes = value.len().to_be_bytes();

            buf.extend_from_slice(&key_length_bytes);
            buf.extend_from_slice(&value_length_bytes);

            buf.extend_from_slice(&key.as_bytes());
            buf.extend_from_slice(&value);

            buf
        })
    }

    /// # Example
    /// ```
    /// use tyozo::Memdb;
    ///
    /// let input = &vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 107, 118, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 118, 107];
    ///
    /// let mut deserialized = Memdb::deserialize(input).unwrap();
    ///
    /// let result = deserialized.exec("get k");
    /// assert_eq!(result, Ok(String::from("v")));
    ///
    /// let result = deserialized.exec("get v");
    /// assert_eq!(result, Ok(String::from("k")));
    /// ```
    pub fn deserialize(input: &[u8]) -> Result<Memdb, String> {
        let mut position = 0usize;
        let mut inner = HashMap::new();

        while input.len() > position {
            position += Memdb::deserialize_paier(&mut inner, &input[position..])?;
        }

        Ok(Memdb { inner })
    }

    fn deserialize_paier(inner: &mut MemdbInner, input: &[u8]) -> Result<usize, String> {
        let key_position = 8;
        let key_length = match input.get(0..key_position) {
            None => return Err(String::from("ERR invalid database format")),
            Some(bytes) => usize::from_be_bytes([
                // FIXME 絶対なにかいい方法がある！！
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]),
        };

        let value_position = key_position + 8;
        let value_length = match input.get(key_position..value_position) {
            None => return Err(String::from("ERR invalid database format")),
            Some(bytes) => usize::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]),
        };

        let key_position = value_position + value_length;
        // TODO add error handle
        let key =
            String::from_utf8(input.get(value_position..key_position).unwrap().to_vec()).unwrap();

        // TODO add error handle
        let value_position = key_position + key_length;
        let value = input.get(key_position..value_position).unwrap().to_vec();

        inner.insert(key, value);

        Ok(value_position)
    }
}
