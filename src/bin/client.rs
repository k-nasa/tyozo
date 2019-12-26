use std::io::prelude::*;
use std::io::Write;
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;

    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();

        let input: String = read();

        if &input == "exit" {
            println!("exit");

            break;
        }

        writeln!(stream, "{}", input)?;
        stream.flush()?;

        let mut buf = String::new();

        let mut stream_reader = std::io::BufReader::new(&stream);
        stream_reader.read_line(&mut buf)?;

        println!("{}", buf);
    }

    Ok(())
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
