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


#[derive(PartialEq,Clone, Copy)]
pub enum TokenType {
    Unknown,
    Heading,
    Letter,
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Unknown => write!(f, "TokenType::Unknown"),
            TokenType::Heading => write!(f, "TokenType::Heading"),
            TokenType::Letter => write!(f, "TokenType::Letter"),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Unknown => write!(f, "TokenType::Unknown"),
            TokenType::Heading => write!(f, "TokenType::Heading"),
            TokenType::Letter => write!(f, "TokenType::Letter"),
        }
    }
}


#[cfg(test)]
mod tests {
    // use claim::{assert_ok, assert_err};

    use claim::assert_matches;

    use super::{Token, TokenType};

    #[test]
    fn test_create_token_heading() {
        let token = Token::new(TokenType::Heading, "".to_string());
        assert_matches!(token.token_type, TokenType::Heading);
    }

    #[test]
    fn test() {
        assert_eq!(true, true)
    }
}
