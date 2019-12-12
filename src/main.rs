use ansi_term::Colour::Red;
use std::io::Write;
use tyozo::Memdb;

fn main() -> Result<(), String> {
    let mut db = Memdb::new();

    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();

        let input: String = read();

        if &input == "exit" {
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
