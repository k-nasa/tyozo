use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex, RwLock};

use crate::command::Command;
use crate::locks::Locks;
use crate::memdb::Memdb;
use crate::parser;
use crate::transaction::Transaction;

pub struct Executor {
    inner: Arc<ExecutorInner>,
    mode: Mode,
    transaction: Transaction,
}

struct ExecutorInner {
    log_file: Mutex<File>,
    db_file: Mutex<File>,
    locks: Mutex<Locks>,
    memdb: RwLock<Memdb>,
}

#[derive(Debug)]
enum Mode {
    Nornal,
    Transaction,
}

impl Executor {
    pub fn new(log_file: File, db_file: File, memdb: Memdb, locks: Locks) -> Executor {
        let log_file = Mutex::new(log_file);
        let db_file = Mutex::new(db_file);
        let locks = Mutex::new(locks);
        let memdb = RwLock::new(memdb);

        let inner = Arc::new(ExecutorInner {
            log_file,
            db_file,
            locks,
            memdb,
        });

        let mode = Mode::Nornal;
        let transaction = Transaction::new();

        Executor {
            inner,
            mode,
            transaction,
        }
    }

    pub fn exec<S: Into<String>>(
        &mut self,
        input: S,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let input = input.into();
        let command = parser::parse(input)?;

        // FIXME lock取得時のunwrap祭りをどうにかする
        if command == Command::Shutdown {
            {
                // lockをなる早で開放するためにブロックで囲っている
                let serialized = self.inner.memdb.read().unwrap().serialize();
                let mut db_file = self.inner.db_file.lock().unwrap();

                db_file.write_all(&serialized)?;
                db_file.flush()?;
            }

            let log_file = self.inner.log_file.lock().unwrap();
            log_file.set_len(0)?;

            return Ok("shutdown!!".to_string());
        }

        if command == Command::Multi {
            self.as_transaction_mode();
            return Ok("Start transaction".to_owned());
        }

        let output = match self.mode {
            Mode::Nornal => self.exec_command_normal_mode(command),
            Mode::Transaction => self.exec_command_transaction_mode(command),
        }?;

        Ok(output)
    }

    fn exec_command_normal_mode(
        &self,
        command: Command,
    ) -> Result<String, Box<dyn std::error::Error>> {
        writeln!(
            self.inner.log_file.lock().unwrap(),
            "{}",
            command.to_string()
        )?;

        // TODO
        // execが何かによらず write lockを取得してしまっている
        // commandをW R 区別する。などの方法で write lockを常に取らないようにしたい
        let output = self.inner.memdb.write().unwrap().exec_command(command)?;

        Ok(output)
    }

    fn exec_command_transaction_mode(
        &mut self,
        command: Command,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let output =
            self.transaction
                .exec_command(command.clone(), &self.inner.locks, &self.inner.memdb)?;

        if command == Command::Exec || command == Command::Abort {
            self.as_normal_mode();
            self.transaction.clear_lock(&self.inner.locks);
        }

        Ok(output)
    }

    fn as_normal_mode(&mut self) {
        self.mode = Mode::Nornal;
    }

    fn as_transaction_mode(&mut self) {
        self.mode = Mode::Transaction;
    }
}

impl Clone for Executor {
    fn clone(&self) -> Self {
        Executor {
            inner: self.inner.clone(),
            mode: Mode::Nornal,
            transaction: Transaction::new(),
        }
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        self.transaction.clear_lock(&self.inner.locks);
    }
}
