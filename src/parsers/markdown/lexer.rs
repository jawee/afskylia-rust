use crate::parsers::markdown::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
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
            read_position: 1,
            ch: input.as_bytes()[0] as char,

        };

        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            '#' => Token::new(TokenType::Heading1, "".to_string()),
            _ => Token::new(TokenType::Unknown, "".to_string()),
        };

        self.read_char();

        println!("returning {}", tok.token_type);
        return tok;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {

    use crate::parsers::markdown::TokenType;

    use super::Lexer;
    use claim::{assert_ok, assert_err, assert_matches};

    #[test]
    fn test_next_token() {
        let input = "#";

        let mut lexer = Lexer::new(&input).unwrap();

        let tok = lexer.next_token();
        // match tok.token_type {
        //     TokenType::Heading1 => {},
        //     _ => assert_
        println!("tokentype = {}", tok.token_type);
        assert_matches!(tok.token_type, TokenType::Heading1);
    }

    #[test]
    fn can_initialize_new_lexer() {
        let input = "# Hello".to_string();
        assert_ok!(Lexer::new(&input));
    }

    #[test]
    fn lexer_new_empty_input_not_ok() {
        let input = "".to_string();
        assert_err!(Lexer::new(&input));
    }
}
