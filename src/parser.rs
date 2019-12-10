use super::lexer::Lexer;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Command {
    Set { key: String, value: String },
    SetNX { key: String, value: String },
    Get { key: String },
    Del { keys: Vec<String> },
}

pub fn parse<S: Into<String>>(input: S) -> Result<Command, String> {
    let input = split_input(input)?;
    parse_to_commnad(input)
}

fn parse_to_commnad(input: SplitedCommand) -> Result<Command, String> {
    let command_name = match input.get(0) {
        None => return Err(String::from("not input command name")),
        Some(c) => c,
    };

    let command = match &command_name.as_str() {
        &"set" => parse_set_command(input)?,
        &"get" => parse_get_command(input)?,
        &"setnx" => parse_setnx_command(input)?,
        &"del" => parse_del_command(input)?,
        _ => return Err(String::from("unsupport command")),
    };

    Ok(command)
}

fn parse_get_command(input: SplitedCommand) -> Result<Command, String> {
    let key = match input.get(1) {
        None => return Err(String::from("not input key")),
        Some(k) => k.to_string(),
    };

    if input.len() > 2 {
        return Err(String::from("Invalid arguments"));
    }

    Ok(Command::Get { key })
}

fn parse_setnx_command(input: SplitedCommand) -> Result<Command, String> {
    let key = match input.get(1) {
        None => return Err(String::from("not input key")),
        Some(k) => k.to_string(),
    };

    let value = match input.get(2) {
        None => return Err(String::from("not input value")),
        Some(v) => v.to_string(),
    };

    if input.len() > 3 {
        return Err(String::from("Invalid arguments"));
    }

    Ok(Command::SetNX { key, value })
}

fn parse_set_command(input: SplitedCommand) -> Result<Command, String> {
    let key = match input.get(1) {
        None => return Err(String::from("not input key")),
        Some(k) => k.to_string(),
    };

    let value = match input.get(2) {
        None => return Err(String::from("not input value")),
        Some(v) => v.to_string(),
    };

    if input.len() > 3 {
        return Err(String::from("Invalid arguments"));
    }

    Ok(Command::Set { key, value })
}

fn parse_del_command(input: SplitedCommand) -> Result<Command, String> {
    if input.len() < 2 {
        return Err(String::from(
            "ERR wrong number of arguments for 'del' command",
        ));
    }

    Ok(Command::Del {
        keys: input[1..].to_vec(),
    })
}

type SplitedCommand = Vec<String>;

fn split_input<S: Into<String>>(input: S) -> Result<SplitedCommand, String> {
    let mut lexer = Lexer::new(input.into());

    let mut splited_command = vec![];

    lexer.read_char();

    while !lexer.is_end() {
        if lexer.current_ch() == '"' {
            let literal = lexer.read_string_literal();
            splited_command.push(literal?);
        }

        if is_letter(lexer.current_ch()) {
            let ident = lexer.read_identifier();
            splited_command.push(ident);
        }

        lexer.read_char();
    }

    Ok(splited_command)
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

    #[test]
    fn test_split_input_include_delimiter() {
        assert_eq!(
            split_input(r#"set key-hoge value-hoge"#),
            Ok(str_vec_to_splited_command(vec![
                "set",
                "key-hoge",
                "value-hoge"
            ]))
        );

        assert_eq!(
            split_input(r#"set key+hoge value+hoge"#),
            Ok(str_vec_to_splited_command(vec![
                "set",
                "key+hoge",
                "value+hoge"
            ]))
        );

        assert_eq!(
            split_input(r#"set key|hoge value|hoge"#),
            Ok(str_vec_to_splited_command(vec![
                "set",
                "key|hoge",
                "value|hoge"
            ]))
        );
    }

    #[test]
    fn test_split_input_error() {
        assert!(split_input(r#"set key "value"#).is_err());
    }

    #[test]
    fn test_parse_set_command() {
        let input = str_vec_to_splited_command(vec!["set", "key", "value"]);
        let output = parse_set_command(input);
        assert_eq!(
            output,
            Ok(Command::Set {
                key: "key".into(),
                value: "value".into()
            })
        );

        let input = str_vec_to_splited_command(vec!["set", "key"]);
        let output = parse_set_command(input);
        assert!(output.is_err());
        assert_eq!(output.unwrap_err(), "not input value");

        let input = str_vec_to_splited_command(vec!["set"]);
        let output = parse_set_command(input);
        assert!(output.is_err());
        assert_eq!(output.unwrap_err(), "not input key");

        let input = str_vec_to_splited_command(vec!["set", "key", "value", "invalid"]);
        let output = parse_set_command(input);
        assert!(output.is_err());
        assert_eq!(output.unwrap_err(), "Invalid arguments");
    }

    #[test]
    fn test_parse_get_command() {
        let input = str_vec_to_splited_command(vec!["get", "key"]);
        let output = parse_get_command(input);
        assert_eq!(output, Ok(Command::Get { key: "key".into() }));

        let input = str_vec_to_splited_command(vec!["get"]);
        let output = parse_get_command(input);
        assert!(output.is_err());
        assert_eq!(output.unwrap_err(), "not input key");

        let input = str_vec_to_splited_command(vec!["get", "key", "invalid"]);
        let output = parse_get_command(input);
        assert!(output.is_err());
        assert_eq!(output.unwrap_err(), "Invalid arguments");
    }

    #[test]
    fn test_parse_setnx_command() {
        let input = str_vec_to_splited_command(vec!["setnx", "key", "value"]);
        let output = parse_setnx_command(input);
        assert_eq!(
            output,
            Ok(Command::SetNX {
                key: "key".into(),
                value: "value".into()
            })
        );
    }

    #[test]
    fn test_parse_del_command() {
        let input = str_vec_to_splited_command(vec!["del", "key"]);
        let output = parse_del_command(input);
        assert_eq!(
            output,
            Ok(Command::Del {
                keys: vec!["key".into()]
            })
        );

        let input = str_vec_to_splited_command(vec!["del", "key1", "key2", "key3"]);
        let output = parse_del_command(input);
        assert_eq!(
            output,
            Ok(Command::Del {
                keys: str_vec_to_splited_command(vec!["key1", "key2", "key3"])
            })
        );

        let input = str_vec_to_splited_command(vec!["del"]);
        let output = parse_del_command(input);
        assert_eq!(
            output.unwrap_err(),
            String::from("ERR wrong number of arguments for 'del' command")
        );
    }

    fn str_vec_to_splited_command(input: Vec<&str>) -> SplitedCommand {
        input.iter().map(ToString::to_string).collect()
    }
}
