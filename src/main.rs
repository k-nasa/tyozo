mod lexer;
mod parser;

fn main() -> Result<(), String> {
    println!("Hello, world!");
    tyozo("hoge")
}

fn tyozo<S: Into<String>>(input: S) -> Result<(), String> {
    // TODO parse command
    parser::parse(input)?;

    // TODO exec command

    // TODO return exec result

    Ok(())
}

#[test]
fn test_tyozo() {
    assert_eq!(tyozo(""), Ok(()))
}
