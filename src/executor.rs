use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::command::Command;
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

    pub fn exec<S: Into<String>>(
        &mut self,
        input: S,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let command = parser::parse(input)?;

        if command == Command::Shutdown {
            let inner = self.inner.lock().unwrap();
            let serialized = &inner.memdb.serialize();

            let mut db_file = &inner.db_file;

            db_file.write_all(&serialized)?;
            db_file.flush()?;

            // TODO 動くようにする
            // let _log_file = &inner.log_file;
            // file_clear(log_file)?;

            drop(inner);
            return Ok(format!("shutdown!!"));
        }

        // logging
        // writeln!(self.inner.lock().unwrap().log_file, "{}", input.into())?;

        let output = self.inner.lock().unwrap().memdb.exec_command(command)?;

        Ok(output)
    }
}
