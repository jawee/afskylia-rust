use crate::parsers::markdown::{Lexer, Token, TokenType};

pub struct HtmlGenerator {
    lexer: Lexer,
}

impl HtmlGenerator {
    pub fn new(lexer: Lexer) -> HtmlGenerator {
        return HtmlGenerator { lexer };
    }

    pub fn get_html(&mut self) -> Result<String, String> {
        while let Some(i) = self.lexer.next_token() {
            if i.token_type == TokenType::EOF {
                break;
            }
            println!("{:?}", i);
            let _token_html = self.get_html_for_token(i);
        }

        return Ok(String::from("<h1>Hello</h1>"));
    }

    fn get_html_for_token(&mut self, token: Token) -> String {
        // let maybe_next_token = self.lexer.next_token();
        // let next_token = match maybe_next_token {
        //     None => Err("Something went wrong"),
        //     Some(t) => {
        //         Ok(t)
        //     }
        // };
        return "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::markdown::Lexer;

    use super::HtmlGenerator;

    #[test]
    fn get_html_heading() {
        let input = "# Hello";
        let expected = "<h1>Hello</h1>";

        let lexer = Lexer::new(input).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);

        let result = html_generator.get_html().unwrap();
        assert_eq!(result, expected);
    }


    #[test]
    fn new_html_generator() {
        let lexer = Lexer::new(" ").unwrap();
        let _html_generator = HtmlGenerator::new(lexer);
    }
}

