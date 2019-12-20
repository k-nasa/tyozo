use ansi_term::Colour::Red;

use std::io::prelude::*;
use std::io::Write;

use tyozo::Memdb;

const DB_FILE_PATH: &str = "./tyozo.db";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut f = std::fs::File::create(DB_FILE_PATH)?; // create or read
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .read(true)
        .open(DB_FILE_PATH)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let mut db = Memdb::deserialize(contents.as_bytes())?;

    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();

        let input: String = read();

        if &input == "exit" {
            let serialized = db.serialize();

            f.write_all(&serialized)?;
            break;
        }

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
