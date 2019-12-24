use std::fs::File;
use std::sync::{Arc, Mutex};

use crate::memdb::Memdb;
use crate::parser;

#[derive(Clone)]
pub struct Executor {
    inner: Arc<Mutex<ExecutorInner>>,
}

struct ExecutorInner {
    log_file: File,
    db_file: File,
    memdb: Memdb,
}

impl Executor {
    pub fn new(log_file: File, db_file: File, memdb: Memdb) -> Executor {
        let inner = Arc::new(Mutex::new(ExecutorInner {
            log_file,
            db_file,
            memdb,
        }));

        Executor { inner }
    }

    pub fn exec<S: Into<String>>(&mut self, input: S) -> Result<String, String> {
        let command = parser::parse(input)?;

        self.inner.lock().unwrap().memdb.exec_command(command)
    }
}
