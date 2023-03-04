use crate::parsers::markdown::{Lexer, Token, TokenType};

pub struct HtmlGenerator {
    lexer: Lexer,
}

impl HtmlGenerator {
    pub fn new(lexer: Lexer) -> HtmlGenerator {
        return HtmlGenerator { lexer };
    }

    pub fn get_html(&mut self) -> Result<String, String> {
        let mut str_vec: Vec<String> = vec![];
        while let Some(i) = self.lexer.next_token() {
            println!("{}", i.token_type);
            if i.token_type == TokenType::EOF {
                break;
            }

            let token_html = self.get_html_for_token(i)?;
            println!("pushing {}", token_html);
            str_vec.push(token_html);
        }

        return Ok(str_vec.join(""));
    }

    fn get_html_for_token(&mut self, token: Token) -> Result<String, String> {
        let str = match token.token_type {
            TokenType::Heading => {

                let mut heading_level = 1 as usize;
                while let Some(i) = self.lexer.next_token() {
                    if i.token_type == TokenType::Letter && i.literal == " " {
                        break;
                    }
                    heading_level += 1;
                };
                
                let mut str_vec: Vec<String> = vec![format!("<h{}>", heading_level)];
                while let Some(i) = self.lexer.next_token() {
                    if i.token_type == TokenType::EOF || i.token_type == TokenType::LineBreak {
                        str_vec.push(format!("</h{}>", heading_level));
                        break;
                    }
                    str_vec.push(i.literal);
                };
                str_vec.join("")
            },
            TokenType::Letter => token.literal,
            TokenType::EOF => String::from(""),
            _ => todo!(),
        };
        // println!("TokenType: {}. Result {}", next_token.token_type, str);
        return Ok(str);
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::markdown::Lexer;

    use super::HtmlGenerator;

    #[test]
    fn get_one_paragraph() {
        let input = "Lorem ipsum";
        let expected = "<p>Lorem ipsum</p>";

        let lexer = Lexer::new(input).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);

        let result = html_generator.get_html().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_html_heading_2() {
        let input = "## He";
        let expected = "<h2>He</h2>";

        let lexer = Lexer::new(input).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);

        let result = html_generator.get_html().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_html_heading() {
        let input = "# He";
        let expected = "<h1>He</h1>";

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

