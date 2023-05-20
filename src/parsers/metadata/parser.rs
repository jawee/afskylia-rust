use super::Metadata;

use crate::parsers::metadata::lexer::Token;
use crate::parsers::metadata::lexer::Lexer;

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input).expect("ERROR: Couldn't create lexer from input");
        let cur_token = lexer.next_token();
        let peek_token = lexer.peek_token();
        let parser = Parser {
            lexer,
            cur_token, 
            peek_token,
        };

        return parser;
    }

    fn parse(&mut self) -> Result<Metadata, String> {
        todo!("Implement parser");
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::parsers::metadata::afdatetime::AfDateTime;

    use super::Parser;

    #[test]
    fn can_create_parser() {
        let input = "+++\n\
                     date: 2023-04-08T10:17:00\n\
                     published: true\n\
                     +++\n";

        let mut parser = Parser::new(input);
        let metadata = parser.parse().expect("ERROR: Couldn't parse metadata");

        assert_eq!(metadata.published, true);
        assert_eq!(metadata.date, AfDateTime::from("2023-04-08T10:17:00"));
    }
}
