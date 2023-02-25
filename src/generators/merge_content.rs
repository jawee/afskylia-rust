pub fn merge_content(_layout: &str, _content: &str) -> String {
    return String::from("");
}

#[cfg(test)]
mod tests {
    use super::merge_content;

    #[test]
    fn test() {
        let layout = "".to_string();
        let content = "".to_string();
        assert_eq!(merge_content(&layout, &content), "");
    }
}
