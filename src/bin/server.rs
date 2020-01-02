use ansi_term::Colour::Red;

use std::io::prelude::*;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

use tyozo::utils::fs_utils::open_or_create_file;
use tyozo::Executor;
use tyozo::Locks;
use tyozo::Memdb;

const DB_FILE_PATH: &str = "./tyozo.db";
const LOG_FILE_PATH: &str = "./tyozo.log";

fn handle_client(
    mut stream: TcpStream,
    mut executor: Executor,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut input = String::new();

        let mut read_buf = std::io::BufReader::new(&stream);
        let n = read_buf.read_line(&mut input)?;

        if n == 0 {
            break Ok(());
        }

        let res = match executor.exec(input) {
            Err(e) => format!("(error) {}", Red.bold().paint(e.to_string())),
            Ok(s) => s.to_string(),
        };

        writeln!(stream, "{}", res)?;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_file = open_or_create_file(DB_FILE_PATH)?;
    let log_file = open_or_create_file(LOG_FILE_PATH)?;
    let db = Memdb::restore(DB_FILE_PATH, LOG_FILE_PATH)?;

    let listener = TcpListener::bind("127.0.0.1:3333")?;

    let executor = Executor::new(log_file, db_file, db, Locks::new());

    for stream in listener.incoming() {
        let executor = executor.clone();
        std::thread::spawn(|| match handle_client(stream.unwrap(), executor) {
            Ok(()) => (),
            Err(e) => eprintln!("{}", e),
        });
    }

    Ok(())
}
