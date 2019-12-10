use super::lexer::Lexer;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum Command {
    Set { key: String, value: String },
}

type SplitedCommand = Vec<String>;

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
    Ok(Command::Set {
        key: input[1].clone(),
        value: input[2].clone(),
    })
}

fn is_letter(ch: char) -> bool {
    const CHS: [char; 3] = ['|', '-', '+'];
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || CHS.iter().any(|c| &ch == c)
}

#[cfg(test)]
mod test {
    use super::*;

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

    fn str_vec_to_splited_command(input: Vec<&str>) -> SplitedCommand {
        input.iter().map(ToString::to_string).collect()
    }
}
