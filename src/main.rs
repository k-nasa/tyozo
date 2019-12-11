mod lexer;
mod parser;

fn main() -> Result<(), String> {
    tyozo("hoge")
}

fn tyozo<S: Into<String>>(input: S) -> Result<(), String> {
    parser::parse(input)?;

    // TODO exec command

    // TODO return exec result

    Ok(())
}
