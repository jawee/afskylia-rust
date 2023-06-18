
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Delim,
    Key,
    Value,
    EOF,
    INVALID,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
    prev: Option<char>,
}


impl Lexer {
    pub fn new(input: &str) -> Result<Lexer, String> {
        if input.is_empty() {
            return Err("Input is required".to_string());
        }

        return Ok(Self::new_lexer_from_input(input));
    }

    fn new_lexer_from_input(input: &str) -> Lexer {
        let lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None,
            prev: None,
        };

        return lexer;
    }

    fn read_char(&mut self) {
        self.prev = self.ch;
        self.ch = self.input.chars().nth(self.read_position);

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();

        let token = match self.ch {
            None => Token { token_type: TokenType::EOF, literal: None },
            Some('+') => {
                while self.ch.unwrap() == '+' {
                    self.read_char();
                }
                if self.ch.unwrap() == '\n' || self.ch.is_none() {
                }
                Token { token_type: TokenType::Delim, literal: None }
            }
            Some(_) => {
                let mut literal = String::new();
                let mut end_char = Some(':');
                let mut token_type = TokenType::Key;

                if self.prev.is_some() && self.prev == Some(':') {
                    end_char = Some('\n');
                    token_type = TokenType::Value;
                }

                while self.ch.is_some() && self.ch != end_char {
                    literal.push(self.ch.unwrap());
                    self.read_char();
                }

                Token { token_type, literal: Some(literal) }
            }
        };

        return token;
    }
}

#[cfg(test)]
mod lexer_tests {
    use claim::{assert_ok, assert_err};

    use crate::parsers::metadata::lexer::{Lexer, TokenType};

    #[test]
    fn new_lexer_ok() {
        let input = "+++\n\
                     date: 2023-04-08T10:17:00\n\
                     published: true\n\
                     +++\n";

        assert_ok!(Lexer::new(&input));
    }

    #[test]
    fn lexer_next_token_eof() {
        let input = " ";

        let mut lexer = Lexer::new(input).expect("Couldn't create lexer");
        lexer.next_token();
        let tok = lexer.next_token();
        assert_eq!(tok.token_type, TokenType::EOF); 
    }

    #[test]
    fn lexer_test_input() {
        let input = "+++\n\
                     date: 2023-04-08T10:17:00\n\
                     published: true\n\
                     +++\n";
        let mut lexer = Lexer::new(input).expect("Couldn't create lexer");  

        let expected = vec![TokenType::Delim, TokenType::Key, TokenType::Value,
        TokenType::Key, TokenType::Value, TokenType::Delim, TokenType::EOF];

        for (i, _) in expected.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, expected[i]);
        }
    }

    #[test]
    fn lexer_new_empty_input_not_ok() {
        let input = "".to_string();
        assert_err!(Lexer::new(&input));
    }
}
