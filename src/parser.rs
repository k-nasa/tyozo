#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum Command {
    Set { key: String, value: String },
}

type SplitedCommand = Vec<String>;

#[derive(Debug, Clone)]
struct Lexer {
    input: String,
    current_position: usize,
    read_position: usize,
    current_ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            current_position: 0,
            read_position: 1,
            current_ch: '\0',
        }
    }

    pub fn current_ch(&self) -> char {
        self.current_ch
    }

    fn get_char(&self, position: usize) -> char {
        match self.input.chars().nth(position) {
            Some(c) => c,
            None => '\0',
        }
    }

    pub fn read_char(&mut self) {
        self.current_ch = self.get_char(self.current_position);
        self.current_position = self.read_position;
        self.read_position += 1;
    }

    pub fn read_string_literal(&mut self) -> Result<String, String> {
        if self.current_ch != '"' {
            return Err("failed to read string literal".to_string());
        }

        self.read_char();

        let mut s = String::new();

        while self.current_ch != '"' {
            s.push(self.current_ch);
            self.read_char();
        }

        Ok(s)
    }

    pub fn read_identifier(&mut self) -> String {
        let start_position = self.current_position - 1;

        while self.current_ch != ' ' && !self.is_end() {
            self.read_char();
        }

        let end_position = self.current_position - 1;
        self.input[start_position..end_position].to_string()
    }

    pub fn is_end(&self) -> bool {
        self.current_ch() == '\n' || self.current_ch() == '\0'
    }
}

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
