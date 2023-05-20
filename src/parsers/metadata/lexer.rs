
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Plus,
    Letter,
    NewLine,
    EOF
}

pub struct Token {
    token_type: TokenType,
    literal: Option<char>
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

    pub fn peek_token(&mut self) -> Token {
        let ch = self.input.chars().nth(self.read_position);
        let tok = match ch {
            None => TokenType::EOF,
            Some('+') => TokenType::Plus,
            Some('\n') => TokenType::NewLine,
            Some(_) => TokenType::Letter
        };

        return Token { token_type: tok, literal: ch };
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();

        let tok = match self.ch {
            None => TokenType::EOF,
            Some('+') => TokenType::Plus,
            Some('\n') => TokenType::NewLine,
            Some(_) => TokenType::Letter
        };

        return Token { token_type: tok, literal: self.ch };
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
    fn lexer_next_token() {
        let inputs = vec!["+", "\n", "d", "2"];

        let expected = vec![TokenType::Plus, TokenType::NewLine,
                           TokenType::Letter, TokenType::Letter];

        for (i, input) in inputs.iter().enumerate() {
            let mut lexer = Lexer::new(input).expect("Couldn't create lexer");
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
