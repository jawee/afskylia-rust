mod afdatetime;
mod parser;
mod lexer;

use std::{fmt::Display, time::SystemTime};

use self::afdatetime::AfDateTime;

#[derive(Debug, PartialEq)]
pub struct Metadata {
    date: AfDateTime,
    published: bool,
}

impl Metadata {
    fn new(date: AfDateTime, published: bool) -> Self {
        return Metadata {
            date,
            published,
        };
    }
}

impl Default for Metadata {
    fn default() -> Self {
        return Metadata {
            date: AfDateTime::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("ERROR: Couldn't get systemtime").as_secs() as usize),
            published: true,
        };
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.date, self.published)
    }
}


impl From<&str> for Metadata {
    fn from(value: &str) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod metadata_tests {
    use std::time::SystemTime;

    use crate::parsers::metadata::{Metadata, afdatetime::AfDateTime};

    #[test]
    fn metadata_default() {
        let metadata = Metadata::default();

        let datetime = AfDateTime::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("ERROR: Couldn't get systemtime").as_secs() as usize);

        assert_eq!(metadata.published, true);
        assert_eq!(metadata.date, datetime);
    }

    #[test]
    fn metadata_new_from_str() {
        let input = "+++\n\
                     date: 2023-04-08T10:17:00\n\
                     published: true\n\
                     +++\n";

        let metadata = Metadata::from(input);

        assert_eq!(metadata.published, true);
        assert_eq!(metadata.date, AfDateTime::from("2023-04-08T10:17:00"));
    }
}
