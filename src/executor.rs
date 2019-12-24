use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::command::Command;
use crate::memdb::Memdb;
use crate::parser;

#[derive(Clone)]
pub struct Executor {
    inner: Arc<Mutex<ExecutorInner>>,
    mode: Mode,
}

struct ExecutorInner {
    log_file: File,
    db_file: File,
    memdb: Memdb,
}

#[derive(Clone)]
enum Mode {
    Nornal,
    Transaction,
}

impl Executor {
    pub fn new(log_file: File, db_file: File, memdb: Memdb) -> Executor {
        let inner = Arc::new(Mutex::new(ExecutorInner {
            log_file,
            db_file,
            memdb,
        }));

        let mode = Mode::Nornal;

        Executor { inner, mode }
    }

    pub fn exec<S: Into<String>>(
        &mut self,
        input: S,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let input = input.into();
        let command = parser::parse(input)?;

        if command == Command::Shutdown {
            let inner = self.inner.lock().unwrap();
            let serialized = &inner.memdb.serialize();

            let mut db_file = &inner.db_file;

            db_file.write_all(&serialized)?;
            db_file.flush()?;

            let log_file = &inner.log_file;
            log_file.set_len(0)?;

            drop(inner);
            return Ok("shutdown!!".to_string());
        }

        if command == Command::Multi {
            self.to_transaction_mode();
            return Ok("Start transaction".to_owned());
        }

        let output = match self.mode {
            Mode::Nornal => self.exec_command_normal_mode(command),
            Mode::Transaction => todo!(),
        }?;

        Ok(output)
    }

    fn exec_command_normal_mode(
        &self,
        command: Command,
    ) -> Result<String, Box<dyn std::error::Error>> {
        writeln!(
            self.inner.lock().unwrap().log_file,
            "{}",
            command.to_string()
        )?;

        let output = self.inner.lock().unwrap().memdb.exec_command(command)?;

        Ok(output)
    }

    fn _exec_command_transaction_mode(
        &self,
        _command: Command,
    ) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }

    fn _to_normal_mode(&mut self) {
        self.mode = Mode::Nornal;
    }

    fn to_transaction_mode(&mut self) {
        self.mode = Mode::Transaction;
    }
}
