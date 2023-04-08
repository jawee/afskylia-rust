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

        let mut i = self.lexer.next_token();
        while i.token_type != TokenType::EOF {
            if i.token_type == TokenType::EOF {
                break;
            }

            let token_html = self.get_html_for_token(i)?;
            str_vec.push(token_html);

            i = self.lexer.next_token();
        }

        return Ok(str_vec.join(""));
    }

    fn get_html_for_token(&mut self, token: Token) -> Result<String, String> {
        let str = match token.token_type {
            TokenType::Heading => {

                let mut heading_level = 1 as usize;

                let mut i = self.lexer.next_token();
                while i.token_type == TokenType::Heading {
                    heading_level += 1;
                    i = self.lexer.next_token();
                };
                
                let mut str_vec: Vec<String> = vec![format!("<h{}>", heading_level)];

                while i.token_type != TokenType::EOF {
                    if i.token_type == TokenType::EOF || i.token_type == TokenType::LineBreak {
                        break;
                    }
                    str_vec.push(i.literal);
                    i = self.lexer.next_token();
                };
                str_vec.push(format!("</h{}>", heading_level));
                str_vec.join("")
            },
            TokenType::Letter => {
                let mut str_vec: Vec<String> = vec![format!("<p>{}", token.literal)];
                let mut next_token = self.lexer.next_token();
                while next_token.token_type != TokenType::EOF {
                    let peek_token = self.lexer.peek_next_token();

                    if next_token.token_type == TokenType::LineBreak 
                        && 
                        (peek_token.token_type == TokenType::LineBreak 
                         || 
                         peek_token.token_type == TokenType::EOF
                         || 
                         peek_token.token_type == TokenType::OrderedItem
                        ) {
                        break;
                    }
                    

                    if next_token.token_type == TokenType::RBracket {
                        str_vec.push(self.generate_link_html());
                    } else if next_token.token_type == TokenType::Bang {
                        str_vec.push(self.generate_image_html());
                    }
                    else {
                        str_vec.push(next_token.literal);
                    }
                    next_token = self.lexer.next_token();
                };
                str_vec.push(format!("</p>"));
                str_vec.join("")
            },
            TokenType::EOF => String::from(""),
            TokenType::OrderedItem => {
                let mut str_vec: Vec<String> = vec![format!("<ol><li>")];
                let mut i = self.lexer.next_token();
                while i.token_type != TokenType::EOF 
                    && 
                    !(i.token_type == TokenType::LineBreak 
                      && (self.lexer.peek_next_token().token_type == TokenType::LineBreak || 
                          self.lexer.peek_next_token().token_type == TokenType::EOF)) {

                    if i.token_type == TokenType::LineBreak {
                        str_vec.push(String::from("</li><li>"));
                        self.lexer.next_token();
                        i = self.lexer.next_token();
                        continue;
                    }
                    str_vec.push(i.literal);
                    i = self.lexer.next_token();
                };
                str_vec.push(format!("</li></ol>"));
                str_vec.join("")
            },
            TokenType::RBracket => {
                self.generate_link_html()
            },
            TokenType::Bang => {
                self.generate_image_html()
            }
            TokenType::LineBreak => String::from(""),
            _ => {
                unreachable!("Hit _ in html.rs, shouldn't happen");
            },
        };
        return Ok(str);
    }

    fn generate_link_html(&mut self) -> String {
        let mut i = 1;
        let mut peek_token = self.lexer.peek_nth_token(i);
        let mut is_link = false;
        loop {
            if peek_token.token_type == TokenType::EOF {
                break;
            }

            if peek_token.token_type == TokenType::LineBreak {
                break;
            }

            if peek_token.token_type == TokenType::LBracket && self.lexer.peek_nth_token(i+1).token_type == TokenType::RParen {
                //this is an actual link
                is_link = true;
                break;
            }

            i += 1;
            peek_token = self.lexer.peek_nth_token(i);
        }

        let mut res = String::default();
        if is_link {
            let mut next_token = self.lexer.next_token();
            let mut title = String::default();
            while next_token.token_type != TokenType::LBracket {
                title.push_str(&next_token.literal);
                next_token = self.lexer.next_token();
            }

            let mut href = String::default();
            self.lexer.next_token(); // (
            next_token = self.lexer.next_token();
            while next_token.token_type != TokenType::LParen {
                href.push_str(&next_token.literal);
                next_token = self.lexer.next_token();
            }
            res = format!("<a href=\"{href}\">{title}</a>");
        } 

        return res;
    }

    fn generate_image_html(&mut self) -> String {
        let mut i = 1;
        let mut peek_token = self.lexer.peek_nth_token(i);
        let mut is_image = false;
        loop {
            if peek_token.token_type == TokenType::EOF {
                break;
            }

            if peek_token.token_type == TokenType::LineBreak {
                break;
            }

            if peek_token.token_type == TokenType::LBracket && self.lexer.peek_nth_token(i+1).token_type == TokenType::RParen {
                //this is an actual link
                is_image = true;
                break;
            }

            i += 1;
            peek_token = self.lexer.peek_nth_token(i);
        }

        let mut res = String::from("!");
        if is_image {
            self.lexer.next_token(); // [
            let mut next_token = self.lexer.next_token();
            let mut alt_text = String::default();
            while next_token.token_type != TokenType::LBracket {
                alt_text.push_str(&next_token.literal);
                next_token = self.lexer.next_token();
            }

            let mut href = String::default();
            println!("skip char: {}", self.lexer.peek_next_token().literal);
            self.lexer.next_token(); // (
            next_token = self.lexer.next_token();
            while next_token.token_type == TokenType::Letter && next_token.literal != " " {
                href.push_str(&next_token.literal);
                next_token = self.lexer.next_token();
            }
            let mut title = String::default();
            next_token = self.lexer.next_token(); // space
            while next_token.token_type != TokenType::LParen {
                title.push_str(&next_token.literal);
                next_token = self.lexer.next_token();
            }
            res = format!("<img src=\"{href}\" alt=\"{alt_text}\" title=\"{title}\">");
        } 

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::markdown::Lexer;

    use super::HtmlGenerator;

    #[test]
    fn get_image_in_paragraph() {
        let input = "a ![a](b c) b";
        let expected = "<p>a <img src=\"b\" alt=\"a\" title=\"c\"> b</p>";

        let lexer = Lexer::new(input).expect("ERROR: Couldn't create lexer");

        let mut html_generator = HtmlGenerator::new(lexer);
        let result = html_generator.get_html().expect("ERROR: Couldn't get html");
        assert_eq!(result, expected);
    }

    #[test]
    fn get_image() {
        let input = "![a](b c)";

        let expected = "<img src=\"b\" alt=\"a\" title=\"c\">";

        let lexer = Lexer::new(input).expect("ERROR: Couldn't create lexer");

        let mut html_generator = HtmlGenerator::new(lexer);
        let result = html_generator.get_html().expect("ERROR: Couldn't get html");
        assert_eq!(result, expected);
    }

    #[test]
    fn get_link_in_paragraph() {
        let input = "A [a](b) b";

        let expected = "<p>A <a href=\"b\">a</a> b</p>";

        let lexer = Lexer::new(input).expect("ERROR: Couldn't create lexer");

        let mut html_generator = HtmlGenerator::new(lexer);
        let result = html_generator.get_html().expect("ERROR: Couldn't get html");
        assert_eq!(result, expected);
    }

    #[test]
    fn get_link() {
        let input = "[a](b)";

        let expected = "<a href=\"b\">a</a>";

        let lexer = Lexer::new(input).expect("ERROR: Couldn't create lexer");

        let mut html_generator = HtmlGenerator::new(lexer);
        let result = html_generator.get_html().expect("ERROR: Couldn't get html");
        assert_eq!(result, expected);
    }

    #[test]
    fn get_heading_with_paragraph_and_ordered_list() {
        let input = "# He\n\
                     Lorem ipsum\n\
                     1. A\n\
                     2. B";
        let expected = "<h1>He</h1><p>Lorem ipsum</p><ol><li>A</li><li>B</li></ol>";

        let lexer = Lexer::new(input).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);

        let result = html_generator.get_html().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_ordered_list_eof_2() {
        let input = "1. A\n\
                     2. B";
        let expected = "<ol><li>A</li><li>B</li></ol>";

        let lexer = Lexer::new(input).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);

        let result = html_generator.get_html().unwrap();
        assert_eq!(result, expected);
    }
    #[test]
    fn get_ordered_list_eof() {
        let input = "1. A\n\
                     2. B\n\
                     ";
        let expected = "<ol><li>A</li><li>B</li></ol>";

        let lexer = Lexer::new(input).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);

        let result = html_generator.get_html().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_ordered_list() {
        let input = "1. A\n\
                     2. B\
                     ";
        let expected = "<ol><li>A</li><li>B</li></ol>";

        let lexer = Lexer::new(input).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);

        let result = html_generator.get_html().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_paragraph_with_number_and_hash() {
        let input = "Lorem ipsum 1#";
        let expected = "<p>Lorem ipsum 1#</p>";

        let lexer = Lexer::new(input).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);

        let result = html_generator.get_html().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_heading_with_paragraph() {
        let input = "# Heading\n\
                     Lorem ipsum\n\
                     \n\
                     Lorem ipsum";
        let expected = "<h1>Heading</h1><p>Lorem ipsum</p><p>Lorem ipsum</p>";

        let lexer = Lexer::new(input).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);

        let result = html_generator.get_html().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_two_paragraphs() {
        let input = "Lorem ipsum\n\n\
                     Lorem ipsum";
        let expected = "<p>Lorem ipsum</p><p>Lorem ipsum</p>";

        let lexer = Lexer::new(input).unwrap();
        let mut html_generator = HtmlGenerator::new(lexer);

        let result = html_generator.get_html().unwrap();
        assert_eq!(result, expected);
    }
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

