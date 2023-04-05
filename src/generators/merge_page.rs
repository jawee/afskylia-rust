#[derive(Debug)]
pub struct MergePage(String);

impl MergePage {
    pub fn parse(layout: &str, content: &str) -> Result<MergePage, String> {
        if layout.is_empty() {
            return Err(format!("layout is empty"));
        }

        let res = layout.replace("{content}", content);
        return Ok(Self(res.to_string()));
    }
    pub fn get_page(&self) -> &str {
        return &self.0;
    }
}

impl ToString for MergePage {
    fn to_string(&self) -> String {
        return String::from(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_ok, assert_err};
    use super::MergePage;

    #[test]
    fn parse_returns_expected_page() {
        let layout = r#"
            <html>
            <head><title>Hello</title>
            <body>
            {content}
            </body>
            </html>
            "#.to_string();
        let content = "<h1>Hello</h1>".to_string();

        let expected = r#"
            <html>
            <head><title>Hello</title>
            <body>
            <h1>Hello</h1>
            </body>
            </html>
            "#.to_string();

        let result = MergePage::parse(&layout, &content).unwrap();
        assert_eq!(result.get_page(), expected);
    }

    #[test]
    fn parse_layout_merged_with_content_ok() {
        let layout = r#"
            <html>
            <head><title>Hello</title>
            <body>
            {content}
            </body>
            </html>
            "#.to_string();
        let content = "<h1>Hello</h1>".to_string();

        let _expected = r#"
            <html>
            <head><title>Hello</title>
            <body>
            <h1>Hello</h1>
            </body>
            </html>
            "#.to_string();

        assert_ok!(MergePage::parse(&layout, &content));
    }

    #[test]
    fn parse_valid_layout_is_ok() {
        let layout = "layout".to_string();
        let content = "".to_string();
        assert_ok!(MergePage::parse(&layout, &content));
    }

    #[test]
    fn parse_empty_layout_is_rejected() {
        let layout = "".to_string();
        let content = "content".to_string();
        assert_err!(MergePage::parse(&layout, &content));
    }
}
