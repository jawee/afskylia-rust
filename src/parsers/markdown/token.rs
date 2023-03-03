use core::fmt;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        return Token { 
            token_type,
            literal,
        };
    }
}


#[derive(PartialEq, Clone, Copy)]
pub enum TokenType {
    EOF,
    Unknown,
    Heading,
    Letter,
    LineBreak,
    OrderedListBegin,
    OrderedListEnd,
    Item,
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::EOF => write!(f, "TokenType::EOF"),
            TokenType::Unknown => write!(f, "TokenType::Unknown"),
            TokenType::Heading => write!(f, "TokenType::Heading"),
            TokenType::Letter => write!(f, "TokenType::Letter"),
            TokenType::LineBreak => write!(f, "TokenType::LineBreak"),
            TokenType::OrderedListBegin => write!(f, "TokenType::OrderedListBegin"),
            TokenType::OrderedListEnd => write!(f, "TokenType::OrderedListEnd"),
            TokenType::Item => write!(f, "TokenType::Item"),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::EOF => write!(f, "TokenType::EOF"),
            TokenType::Unknown => write!(f, "TokenType::Unknown"),
            TokenType::Heading => write!(f, "TokenType::Heading"),
            TokenType::Letter => write!(f, "TokenType::Letter"),
            TokenType::LineBreak => write!(f, "TokenType::LineBreak"),
            TokenType::OrderedListBegin => write!(f, "TokenType::OrderedListBegin"),
            TokenType::OrderedListEnd => write!(f, "TokenType::OrderedListEnd"),
            TokenType::Item => write!(f, "TokenType::Item"),
        }
    }
}


#[cfg(test)]
mod tests {
    use claim::assert_matches;

    use super::{Token, TokenType};

    #[test]
    fn test_create_letter_token() {
        let literal = String::from("A");
        let token = Token::new(TokenType::Letter, literal);
        assert_matches!(token.token_type, TokenType::Letter);
        assert_eq!(token.literal, String::from("A"));
    }

    #[test]
    fn test_create_heading_token() {
        let token = Token::new(TokenType::Heading, "".to_string());
        assert_matches!(token.token_type, TokenType::Heading);
    }

    #[test]
    fn test() {
        assert_eq!(true, true)
    }
}
