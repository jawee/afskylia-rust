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

    pub fn peek_next_token(&mut self) -> Token {
        let next_ch = self.input.chars().nth(self.read_position);
         
        let tok = match next_ch {
            Some('#') => Token::new(TokenType::Heading, String::from("")),
            Some('\n') => Token::new(TokenType::LineBreak, String::from("")),
            Some(_) => {
                let prev = self.input.chars().nth(self.read_position-1);
                if next_ch.unwrap().is_digit(10) && (prev.is_none() || prev.unwrap() == '\n') {
                    Token::new(TokenType::OrderedItem, String::from(""))
                } else {
                    Token::new(TokenType::Letter, String::from(self.ch.unwrap()))
                }
            },
            None => Token::new(TokenType::EOF, String::from(""))
        };

        return tok;
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();

        let tok = match self.ch {
            Some('#') => {
                if self.prev.is_none() || self.prev.unwrap() == '\n' || self.prev.unwrap() == '#' {
                    if self.peek_next_token().token_type != TokenType::Heading {
                        self.read_char(); //skip the whitespace
                    }
                    return Token::new(TokenType::Heading, String::from(""));
                }
                return Token::new(TokenType::Letter, String::from("#"));
            },
            Some('\n') => Token::new(TokenType::LineBreak, String::from("")),
            Some(_) => {
                if self.ch.unwrap().is_digit(10) && (self.prev.is_none() || self.prev.unwrap() == '\n') {
                    self.read_char(); //to skip the dot. Which means we can only do 1-9 
                    self.read_char(); //skip the whitespace
                    Token::new(TokenType::OrderedItem, String::from(""))
                } else {
                    Token::new(TokenType::Letter, String::from(self.ch.unwrap()))
                }
            },
            None => Token::new(TokenType::EOF, String::from(""))
        };

        return tok;
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
    fn orderedlist_2() {
        let input = "1. A\n\
                     2. B";
        println!("Input: {} end", input);
        let expected = vec![
        TokenType::OrderedItem, TokenType::Letter, TokenType::LineBreak,
        TokenType::OrderedItem, TokenType::Letter, TokenType::EOF];

        let mut lexer = Lexer::new(input).unwrap();
        for e in expected {
            let tok = lexer.next_token();
            println!("actual: {} == expected: {}", tok.token_type, e);
            assert_eq!(tok.token_type, e, "actual: {} == expected: {}", tok.token_type, e);
        }
    }

    #[test]
    fn orderedlist() {
        let input = "1. A\n\
                     2. B\n\
                     ";
        println!("Input: {} end", input);
        let expected = vec![
        TokenType::OrderedItem, TokenType::Letter, TokenType::LineBreak,
        TokenType::OrderedItem, TokenType::Letter, TokenType::LineBreak, TokenType::EOF];

        let mut lexer = Lexer::new(input).unwrap();
        for e in expected {
            let tok = lexer.next_token();
            println!("actual: {} == expected: {}", tok.token_type, e);
            assert_eq!(tok.token_type, e, "actual: {} == expected: {}", tok.token_type, e);
        }
    }

    #[test]
    fn peek_next_token_paragraph_orderedlist() {
        let input = "L\n\
                     1. A";

        let expected = vec![TokenType::Letter, TokenType::LineBreak,
        TokenType::OrderedItem, TokenType::Letter, TokenType::EOF];
        let mut lexer = Lexer::new(&input).unwrap();

        for (i, e) in expected.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, *e);
            if i+1 < expected.len() {
                assert_eq!(lexer.peek_next_token().token_type, expected[i+1]);
            }
        }
    }

    #[test]
    fn peek_next_token_two_linebreaks_orderedlist() {
        let input = "\n\
                     \n\
                     1. A";

        let mut lexer = Lexer::new(&input).unwrap();

        lexer.next_token();
        lexer.next_token();
        let peek_token = lexer.peek_next_token();
        assert_eq!(peek_token.token_type, TokenType::OrderedItem);
    }

    #[test]
    fn peek_next_token_one_linebreak_orderedlist() {
        let input = "\n\
                     1. A";

        let mut lexer = Lexer::new(&input).unwrap();

        lexer.next_token();
        let peek_token = lexer.peek_next_token();
        assert_eq!(peek_token.token_type, TokenType::OrderedItem);
    }

    #[test]
    fn heading_paragraph_orderedlist() {
        let input = "# He\n\
                     Lo\n\
                     1. A\n\
                     2. B";
        let expected = vec![TokenType::Heading, 
        TokenType::Letter, TokenType::Letter, TokenType::LineBreak,
        TokenType::Letter, TokenType::Letter, TokenType::LineBreak,
        TokenType::OrderedItem, TokenType::Letter, TokenType::LineBreak,
        TokenType::OrderedItem, TokenType::Letter, TokenType::EOF];

        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            println!("expected: {}", e);
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, e);
        }
    }

    #[test]
    fn paragraph_followed_by_ordereditem() {
        let input = "Lo\n\
                     1. A";
        let expected = vec![TokenType::Letter, TokenType::Letter,
        TokenType::LineBreak, TokenType::OrderedItem, 
        TokenType::Letter, TokenType::EOF];

        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            println!("expected: {}", e);
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, e);
        }
    }

    #[test]
    fn peek_next_token() {
        let input = "L\n";
        
        let mut lexer = Lexer::new(&input).unwrap();

        let token = lexer.next_token();
        let peek_token = lexer.peek_next_token();

        assert_eq!(token.token_type, TokenType::Letter);
        assert_eq!(peek_token.token_type, TokenType::LineBreak);
    }

    #[test]
    fn paragraph_with_number_and_hash() {
        let input = "Lo12#";

        let expected = vec![TokenType::Letter, TokenType::Letter,
        TokenType::Letter, TokenType::Letter, TokenType::Letter,
        TokenType::EOF];

        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            println!("expected: {}", e);
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, e);
        }
    }
    
    #[test]
    fn two_paragraphs() {
        let input = "Lo\n\
                     \n\
                     um";

        let expected = vec![TokenType::Letter, TokenType::Letter,
        TokenType::LineBreak, TokenType::LineBreak, TokenType::Letter,
        TokenType::Letter];

        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            println!("expected: {}", e);
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, e);
        }
    }

    #[test]
    fn next_token_ordered_list_2() {
        let input = "1. A\n\
                     2. B\n\
                     ";
        let expected = vec![TokenType::OrderedItem, TokenType::Letter,
        TokenType::LineBreak, TokenType::OrderedItem, TokenType::Letter,
        TokenType::LineBreak, TokenType::EOF];

        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            println!("expected: {}", e);
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, e);
        }
    }

    #[test]
    fn next_token_ordered_list_1() {
        let input = "1. A\n\
                     2. B";
        let expected = vec![TokenType::OrderedItem, TokenType::Letter,
        TokenType::LineBreak, TokenType::OrderedItem, TokenType::Letter,
        TokenType::EOF];

        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            println!("expected: {}", e);
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, e);
        }
    }

    #[test]
    fn next_token_ordered_list() {
        let input = "\n\
                     1. a\n\
                     2. b\n\
                     ";
        let expected = vec![TokenType::LineBreak, TokenType::OrderedItem,
        TokenType::Letter, TokenType::LineBreak, TokenType::OrderedItem,
        TokenType::Letter, TokenType::LineBreak, TokenType::EOF];
        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            println!("expected: {}", e);
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, e);
        }
    }

    #[test]
    fn next_token_heading_with_paragraph() {
        let input = "# he\n\
                     text\n\
                     ";

        let expected = vec![TokenType::Heading, 
        TokenType::Letter, TokenType::Letter, TokenType::LineBreak,
        TokenType::Letter, TokenType::Letter, TokenType::Letter,
        TokenType::Letter, TokenType::LineBreak, TokenType::EOF];
        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            println!("expected: {}", e);
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, e);
        }
    }

    #[test]
    fn next_token_heading_with_text() {
        let input = "# He";
        let expected = vec![TokenType::Heading, 
        TokenType::Letter, TokenType::Letter, TokenType::EOF];
        let mut lexer = Lexer::new(&input).unwrap();

        for e in expected {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, e);
        }
    }

    #[test]
    fn next_token_heading_twice() {
        let input = "##";

        let mut lexer = Lexer::new(&input).unwrap();
        let mut tok = lexer.next_token();
        assert_matches!(tok.token_type, TokenType::Heading);
        tok = lexer.next_token();
        assert_matches!(tok.token_type, TokenType::Heading);
    }

    #[test]
    fn next_token_letter_space() {
        let input = " ";

        let mut lexer = Lexer::new(&input).unwrap();
        let tok = lexer.next_token();

        assert_matches!(tok.token_type, TokenType::Letter);
        assert_eq!(tok.literal, String::from(" "));
    }

    #[test]
    fn next_token_linebreak_then_letter() {
        let input = "\n\
                     a";

        let mut lexer = Lexer::new(&input).unwrap();

        assert_matches!(lexer.next_token().token_type, TokenType::LineBreak);
        assert_matches!(lexer.next_token().token_type, TokenType::Letter);
    }

    #[test]
    fn next_token_linebreak() {
        let input = "\n\
                     ";

        let mut lexer = Lexer::new(&input).unwrap();

        assert_matches!(lexer.next_token().token_type, TokenType::LineBreak);
    }

    #[test]
    fn next_token_letter_a() {
        let input = "a";

        let mut lexer = Lexer::new(&input).unwrap();
        let tok = lexer.next_token();

        assert_matches!(tok.token_type, TokenType::Letter);
        assert_eq!(tok.literal, String::from("a"));
    }

    #[test]
    fn next_token_heading() {
        let input = "#";

        let mut lexer = Lexer::new(&input).unwrap();
        let tok = lexer.next_token();

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
