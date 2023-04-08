use std::fmt::Display;

use chrono::prelude::*;

#[derive(Debug)]
pub struct Metadata {
    date: chrono::prelude::DateTime<Utc>,
    published: bool,
}

impl Default for Metadata {
    fn default() -> Self {
        return Metadata {
            date: Utc::now(),
            published: true,
        };
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.date, self.published)
    }
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
}

#[cfg(test)]
mod tests {
    use claim::{assert_ok, assert_err};

    use crate::parsers::metadata::Lexer;

    #[test]
    fn new_lexer_ok() {
        let input = "---\n\
                     date: 2023-04-08T10:17.00Z\n\
                     published: true\n\
                     ---\n";

        assert_ok!(Lexer::new(&input));
    }

    #[test]
    fn lexer_new_empty_input_not_ok() {
        let input = "".to_string();
        assert_err!(Lexer::new(&input));
    }
}
