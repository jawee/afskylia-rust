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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} '{}'", self.token_type, self.literal)
    }
}


#[derive(PartialEq, Clone, Copy)]
pub enum TokenType {
    EOF,
    Unknown,
    Heading,
    Letter,
    LineBreak,
    OrderedItem,
    Item,
    LParen,
    RParen,
    RBracket,
    LBracket,
    Bang,
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::EOF => write!(f, "TokenType::EOF"),
            TokenType::Unknown => write!(f, "TokenType::Unknown"),
            TokenType::Heading => write!(f, "TokenType::Heading"),
            TokenType::Letter => write!(f, "TokenType::Letter"),
            TokenType::LineBreak => write!(f, "TokenType::LineBreak"),
            TokenType::OrderedItem => write!(f, "TokenType::OrderedItem"),
            TokenType::Item => write!(f, "TokenType::Item"),
            TokenType::LParen => write!(f, "TokenType::LParen"),
            TokenType::RParen => write!(f, "TokenType::RParen"),
            TokenType::RBracket => write!(f, "TokenType::RBracket"),
            TokenType::LBracket => write!(f, "TokenType::LBracket"),
            TokenType::Bang => write!(f, "TokenType::Bang"),
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
            TokenType::OrderedItem => write!(f, "TokenType::OrderedItem"),
            TokenType::Item => write!(f, "TokenType::Item"),
            TokenType::LParen => write!(f, "TokenType::LParen"),
            TokenType::RParen => write!(f, "TokenType::RParen"),
            TokenType::RBracket => write!(f, "TokenType::RBracket"),
            TokenType::LBracket => write!(f, "TokenType::LBracket"),
            TokenType::Bang => write!(f, "TokenType::Bang"),
        }
    }
}


#[cfg(test)]
mod tests {
    use claim::assert_matches;

    use super::{Token, TokenType};

    #[test]
    fn create_letter_token() {
        let literal = String::from("A");
        let token = Token::new(TokenType::Letter, literal);
        assert_matches!(token.token_type, TokenType::Letter);
        assert_eq!(token.literal, String::from("A"));
    }

    #[test]
    fn create_heading_token() {
        let token = Token::new(TokenType::Heading, "".to_string());
        assert_matches!(token.token_type, TokenType::Heading);
    }
}
