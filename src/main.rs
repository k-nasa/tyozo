use ansi_term::Colour::Red;

use std::io::prelude::*;
use std::io::Write;

use tyozo::Memdb;

const DB_FILE_PATH: &str = "./tyozo.db";
const LOG_FILE_PATH: &str = "./tyozo.log";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db_file = std::fs::OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open(DB_FILE_PATH)?;

    let mut contents = String::new();
    db_file.read_to_string(&mut contents)?;

    let mut db = Memdb::deserialize(contents.as_bytes())?;

    let mut log_file = std::fs::OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open(LOG_FILE_PATH)?;

    let mut logs = String::new();
    log_file.read_to_string(&mut logs)?;

    logs.lines().for_each(|log| {
        let _ = db.exec(log);
    });

    // initialize log file
    std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(LOG_FILE_PATH)?;

    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();

        let input: String = read();

        if &input == "exit" {
            let serialized = db.serialize();

            db_file.write_all(&serialized)?;
            db_file.flush()?;

            // initialize log file
            std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(LOG_FILE_PATH)?;

            break;
        }

        // logging
        log_file.write_all(&input.as_bytes())?;
        log_file.write_all(&[b'\n'])?;
        log_file.flush()?;

        match db.exec(input) {
            Err(e) => println!("(error) {}", Red.bold().paint(e)),
            Ok(s) => println!("{}", s),
        }
    }

    Ok(())
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
