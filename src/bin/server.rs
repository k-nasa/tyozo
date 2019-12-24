use ansi_term::Colour::Red;

use std::io::prelude::*;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

use tyozo::Executor;
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
            continue;
        }

        // if &input == "shutdown" {
        //     let serialized = db.clone().serialize();
        //
        //     db_file.write_all(&serialized)?;
        //     db_file.flush()?;
        //
        //     file_clear(LOG_FILE_PATH)?;
        //
        //     writeln!(stream, "shutdown")?;
        //     break;
        // }
        //
        // // logging
        // writeln!(log_file, "{}", input)?;

        let res = match executor.exec(input) {
            Err(e) => format!("(error) {}", Red.bold().paint(e)),
            Ok(s) => s.to_string(),
        };

        writeln!(stream, "{}", res)?;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db_file = open_or_create_file(DB_FILE_PATH)?;

    let mut contents = String::new();
    db_file.read_to_string(&mut contents)?;

    let mut db = Memdb::deserialize(contents.as_bytes())?;

    let mut log_file = open_or_create_file(LOG_FILE_PATH)?;

    let mut logs = String::new();
    log_file.read_to_string(&mut logs)?;

    logs.lines().for_each(|log| {
        let _ = db.exec(log);
    });

    file_clear(LOG_FILE_PATH)?;

    let listener = TcpListener::bind("127.0.0.1:3333")?;

    let executor = Executor::new(log_file, db_file, db);

    for stream in listener.incoming() {
        let executor = executor.clone();
        std::thread::spawn(|| match handle_client(stream.unwrap(), executor) {
            Ok(()) => (),
            Err(e) => eprintln!("{}", e),
        });
    }

    Ok(())
}

fn file_clear(path: &str) -> Result<(), std::io::Error> {
    std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .map(|_| ())
}

fn open_or_create_file(path: &str) -> Result<std::fs::File, std::io::Error> {
    std::fs::OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open(path)
}
