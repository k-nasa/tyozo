use super::lexer::Lexer;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Command {
    Set { key: String, value: String },
}

type SplitedCommand = Vec<String>;

pub fn parse<S: Into<String>>(input: S) -> Result<Command, String> {
    let input = split_input(input)?;
    parse_to_commnad(input)
}

fn parse_set_command(input: SplitedCommand) -> Result<Command, String> {
    let key = match input.get(1) {
        None => return Err(String::from("not input command name")),
        Some(k) => k.to_string(),
    };

    let value = match input.get(2) {
        None => return Err(String::from("not input command name")),
        Some(v) => v.to_string(),
    };

    Ok(Command::Set { key, value })
}

fn split_input<S: Into<String>>(input: S) -> Result<SplitedCommand, String> {
    let mut lexer = Lexer::new(input.into());

    let mut splited_command = vec![];

    lexer.read_char();

    while !lexer.is_end() {
        if lexer.current_ch() == '"' {
            let literal = lexer.read_string_literal();
            splited_command.push(literal.unwrap());
        }

        if is_letter(lexer.current_ch()) {
            let ident = lexer.read_identifier();
            splited_command.push(ident);
        }

        lexer.read_char();
    }

    Ok(splited_command)
}

fn parse_to_commnad(input: SplitedCommand) -> Result<Command, String> {
    let command_name = match input.get(0) {
        None => return Err(String::from("not input command name")),
        Some(c) => c,
    };

    let command = match &command_name.as_str() {
        &"set" => parse_set_command(input)?,
        _ => return Err(String::from("unsupport command")),
    };

    Ok(command)
}

fn is_letter(ch: char) -> bool {
    const CHS: [char; 3] = ['|', '-', '+'];
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || CHS.iter().any(|c| &ch == c)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_to_command() {
        let input = str_vec_to_splited_command(vec!["set", "key", "value"]);

        assert_eq!(
            parse_to_commnad(input),
            Ok(Command::Set {
                key: "key".into(),
                value: "value".into()
            })
        )
    }

    #[test]
    fn test_split_input() {
        assert_eq!(
            split_input(r#"set "key" "value hoge""#),
            Ok(str_vec_to_splited_command(vec!["set", "key", "value hoge"]))
        );

        assert_eq!(
            split_input(r#"set "key" value"#),
            Ok(str_vec_to_splited_command(vec!["set", "key", "value"]))
        );

        assert_eq!(
            split_input(r#"set key "value""#),
            Ok(str_vec_to_splited_command(vec!["set", "key", "value"]))
        );

        assert_eq!(
            split_input(r#""set" key value"#),
            Ok(str_vec_to_splited_command(vec!["set", "key", "value"]))
        );

        assert_eq!(
            split_input(r#"set key value"#),
            Ok(str_vec_to_splited_command(vec!["set", "key", "value"]))
        );

        assert_eq!(
            split_input(r#"set key        value"#),
            Ok(str_vec_to_splited_command(vec!["set", "key", "value"]))
        );
    }

    fn str_vec_to_splited_command(input: Vec<&str>) -> SplitedCommand {
        input.iter().map(ToString::to_string).collect()
    }
}
