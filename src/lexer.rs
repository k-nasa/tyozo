#[derive(Debug, Clone)]
pub struct Lexer {
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
