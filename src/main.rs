mod lexer;
mod parser;

fn main() -> Result<(), ()> {
    println!("Hello, world!");
    tyozo("hoge")
}

fn tyozo<S: Into<String>>(_input: S) -> Result<(), ()> {
    // TODO parse command

    // TODO exec command

    // TODO return exec result

    Ok(())
}

#[test]
fn test_tyozo() {
    assert_eq!(tyozo(""), Ok(()))
}
