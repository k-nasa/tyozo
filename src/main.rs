mod parser;

fn main() {
    println!("Hello, world!");
}

fn tyozo<S: Into<String>>(input: S) -> Result<(), ()> {
    // TODO parse command

    // TODO exec command

    // TODO return exec result

    Ok(())
}

#[test]
fn test_tyozo() {
    assert_eq!(tyozo(""), Ok(()))
}
