use crate::parsers::markdown::token::{Token, TokenType};

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

    pub fn next_token(&mut self) -> Option<Token> {
        self.read_char();

        if self.ch.is_none() {
            return Some(Token::new(TokenType::EOF, String::from("")));
        }

        let tok = match self.ch.unwrap() {
            '#' => Token::new(TokenType::Heading, String::from("")),
            '\n' => Token::new(TokenType::LineBreak, String::from("")),
            _ => {
                if self.ch.unwrap().is_digit(10) && self.prev.unwrap() == '\n' {
                    self.read_char(); //to skip the dot
                    Token::new(TokenType::OrderedItem, String::from(""))
                } else {
                    Token::new(TokenType::Letter, String::from(self.ch.unwrap()))
                }
            }
        };

        // println!("returning {}", tok.token_type);
        return Some(tok);
    }

    fn read_char(&mut self) {
        self.prev = self.ch;
        self.ch = self.input.chars().nth(self.read_position);

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
    fn test_next_token_ordered_list() {
        let input = "\n\
                     1. a\n\
                     2. b\n\
                     ";
        let expected = vec![TokenType::LineBreak, TokenType::OrderedItem,
        TokenType::Letter, TokenType::Letter, TokenType::LineBreak,
        TokenType::OrderedItem, TokenType::Letter, TokenType::Letter,
        TokenType::LineBreak, TokenType::EOF];
        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            println!("expected: {}", e);
            let tok = lexer.next_token().unwrap();
            assert_eq!(tok.token_type, e);
        }
    }

    #[test]
    fn test_next_token_heading_with_paragraph() {
        let input = "# he\n\
                     text\n\
                     ";

        let expected = vec![TokenType::Heading, TokenType::Letter,
        TokenType::Letter, TokenType::Letter, TokenType::LineBreak,
        TokenType::Letter, TokenType::Letter, TokenType::Letter,
        TokenType::Letter, TokenType::LineBreak, TokenType::EOF];
        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            println!("expected: {}", e);
            let tok = lexer.next_token().unwrap();
            assert_eq!(tok.token_type, e);
        }
    }

    #[test]
    fn test_next_token_heading_with_text() {
        let input = "# He";
        let expected = vec![TokenType::Heading, TokenType::Letter,
        TokenType::Letter, TokenType::Letter, TokenType::EOF];
        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            let tok = lexer.next_token().unwrap();
            assert_eq!(tok.token_type, e);
        }
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
    fn test_next_token_linebreak_then_letter() {
        let input = "\n\
                     a";

        let mut lexer = Lexer::new(&input).unwrap();

        assert_matches!(lexer.next_token().unwrap().token_type, TokenType::LineBreak);
        assert_matches!(lexer.next_token().unwrap().token_type, TokenType::Letter);
    }

    #[test]
    fn test_next_token_linebreak() {
        let input = "\n\
                     ";

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
