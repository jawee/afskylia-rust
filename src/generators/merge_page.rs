#[derive(Debug)]
pub struct MergePage(String);

impl MergePage {
    pub fn parse(_layout: &str, _content: &str) -> Result<MergePage, String> {
        if _layout.is_empty() {
            return Err(format!("layout is empty"));
        }

        return Ok(Self("".to_string()));
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_ok, assert_err};
    use super::MergePage;


    #[test]
    fn valid_layout_is_ok() {
        let layout = "layout".to_string();
        let content = "".to_string();
        assert_ok!(MergePage::parse(&layout, &content));
    }
    #[test]
    fn empty_layout_is_rejected() {
        let layout = "".to_string();
        let content = "content".to_string();
        assert_err!(MergePage::parse(&layout, &content));
    }
}
