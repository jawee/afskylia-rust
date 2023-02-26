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


#[derive(Clone, Copy)]
pub enum TokenType {
    Unknown,
    Heading1,
    Heading2,
    Paragraph,
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Unknown => write!(f, "TokenType::Unknown"),
            TokenType::Heading1 => write!(f, "TokenType::Heading1"),
            TokenType::Heading2 => write!(f, "TokenType::Heading2"),
            TokenType::Paragraph => write!(f, "TokenType::Paragraph"),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Unknown => write!(f, "TokenType::Unknown"),
            TokenType::Heading1 => write!(f, "TokenType::Heading1"),
            TokenType::Heading2 => write!(f, "TokenType::Heading2"),
            TokenType::Paragraph => write!(f, "TokenType::Paragraph"),
        }
    }
}


#[cfg(test)]
mod tests {
    // use claim::{assert_ok, assert_err};

    use claim::assert_matches;

    use super::{Token, TokenType};

    #[test]
    fn test_create_token_heading1() {
        let token = Token::new(TokenType::Heading1, "".to_string());
        assert_matches!(token.token_type, TokenType::Heading1);
    }

    #[test]
    fn test() {
        assert_eq!(true, true)
    }
}
