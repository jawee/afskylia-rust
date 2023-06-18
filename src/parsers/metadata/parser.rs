use super::Metadata;

use crate::parsers::metadata::afdatetime::AfDateTime;
use crate::parsers::metadata::lexer::Token;
use crate::parsers::metadata::lexer::Lexer;
use crate::parsers::metadata::lexer::TokenType;

struct Parser {
    lexer: Lexer,
    cur_token: Token,
}

impl Parser {
    fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input).expect("ERROR: Couldn't create lexer from input");
        let cur_token = lexer.next_token();
        let parser = Parser {
            lexer,
            cur_token, 
        };

        return parser;
    }

    fn read_next(&mut self) {
        self.cur_token = self.lexer.next_token();
    }

    fn parse(&mut self) -> Result<Metadata, String> {
        let mut date: Option<AfDateTime> = None;
        let mut published: Option<bool> = None;
        while self.cur_token.token_type != TokenType::EOF {
            match self.cur_token.token_type {
                TokenType::Delim => { }
                TokenType::Key => {
                    match self.cur_token.literal.as_ref().unwrap().as_str() {
                        "date" => {
                            self.read_next();
                            match self.cur_token.token_type {
                                TokenType::Value => {
                                    let str = self.cur_token.literal.clone().unwrap();
                                    date = Some(AfDateTime::from(str));
                                }
                                _ => {
                                    return Err(format!("ERROR: Unexpected token: {:?}", self.cur_token));
                                }
                            }
                        }
                        "published" => { 
                            self.read_next();
                            match self.cur_token.token_type {
                                TokenType::Value => {
                                    let str = self.cur_token.literal.clone().unwrap();
                                    published = Some(str.parse::<bool>().unwrap());
                                }
                                _ => {
                                    return Err(format!("ERROR: Unexpected token: {:?}", self.cur_token));
                                }
                            }
                        }
                        _ => {
                            return Err(format!("ERROR: Unexpected key: {:?}", self.cur_token.literal.as_ref().unwrap()));
                        }
                    }
                }
                _ => {
                    return Err(format!("ERROR: Unexpected token: {:?}", self.cur_token.token_type));
                }
            }
            self.read_next();
        }
        return Ok(Metadata::new(date.unwrap(), published.unwrap()));
    }
}

#[cfg(test)]
mod parser_tests {
    use claim::assert_err;

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

    #[test]
    fn parse_error() {
        let input = "++=\n";
        let mut parser = Parser::new(input);
        assert_err!(parser.parse(), "ERROR: Unexpected token: Eq");
    }
}
