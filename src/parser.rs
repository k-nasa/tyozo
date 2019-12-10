#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum Command {
    Set { key: String, value: String },
}

type SplitedCommand = Vec<String>;

fn split_input<S: Into<String>>(input: S) -> Result<SplitedCommand, String> {
    Ok(vec!["set", "key", "value hoge"]
        .iter()
        .map(ToString::to_string)
        .collect())
}

fn parse_to_commnad(input: SplitedCommand) -> Result<Command, String> {
    Ok(Command::Set {
        key: input[1].clone(),
        value: input[2].clone(),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_input() {
        assert_eq!(
            split_input(r#"set "key" "value hoge""#),
            Ok(str_vec_to_splited_command(vec!["set", "key", "value hoge"]))
        )
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
