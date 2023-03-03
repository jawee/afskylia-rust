use crate::parsers::markdown::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
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
        };

        return lexer;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.read_char();

        if self.ch.is_none() {
            return None
        }

        let tok = match self.ch.unwrap() {
            '#' => Token::new(TokenType::Heading, String::from("")),
            '\n' => Token::new(TokenType::LineBreak, String::from("")),
            _ => Token::new(TokenType::Letter, String::from(self.ch.unwrap())),
        };

        println!("returning {}", tok.token_type);
        return Some(tok);
    }

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position);

        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::markdown::TokenType;

    use super::Lexer;
    use claim::{assert_ok, assert_err, assert_matches, assert_none};

    #[test]
    fn test_next_token_heading_with_text() {
        let input = "# He";
        let expected = vec![TokenType::Heading, TokenType::Letter, TokenType::Letter, TokenType::Letter];
        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            let tok = lexer.next_token().unwrap();
            assert_eq!(tok.token_type, e);
        }

        let tok = lexer.next_token();
        assert_none!(tok);
    }

    #[test]
    fn test_next_token_heading_twice() {
        let input = "##";

        let mut lexer = Lexer::new(&input).unwrap();
        let mut tok = lexer.next_token().unwrap();
        assert_matches!(tok.token_type, TokenType::Heading);
        tok = lexer.next_token().unwrap();
        assert_matches!(tok.token_type, TokenType::Heading);
    }

    #[test]
    fn test_next_token_letter_space() {
        let input = " ";

        let mut lexer = Lexer::new(&input).unwrap();
        let tok = lexer.next_token().unwrap();

        assert_matches!(tok.token_type, TokenType::Letter);
        assert_eq!(tok.literal, String::from(" "));
    }

    #[test]
    fn test_next_token_linebreak() {
        let input = r#"
            "#;

        let mut lexer = Lexer::new(&input).unwrap();

        assert_matches!(lexer.next_token().unwrap().token_type, TokenType::LineBreak);
    }

    #[test]
    fn test_next_token_letter_a() {
        let input = "a";

        let mut lexer = Lexer::new(&input).unwrap();
        let tok = lexer.next_token().unwrap();

        assert_matches!(tok.token_type, TokenType::Letter);
        assert_eq!(tok.literal, String::from("a"));
    }

    #[test]
    fn test_next_token_heading() {
        let input = "#";

        let mut lexer = Lexer::new(&input).unwrap();
        let tok = lexer.next_token().unwrap();

        assert_matches!(tok.token_type, TokenType::Heading);
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
