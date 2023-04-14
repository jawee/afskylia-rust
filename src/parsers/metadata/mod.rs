use std::{fmt::Display, collections::HashMap};

use chrono::prelude::*;

#[derive(Debug)]
pub struct Metadata {
    date: chrono::prelude::DateTime<Utc>,
    published: bool,
}

pub struct MyDateTime {
    timestamp: usize,
    day: usize,
    month: usize,
    year: usize,
    hour: usize,
    minute: usize,
    second: usize
}

impl MyDateTime {

    pub fn new(timestamp: usize) -> Self {
        let map = Self::parse_timestamp(timestamp);

        let second = *map.get("second").expect("ERROR: Couldn't get second");
        let minute = *map.get("minute").expect("ERROR: Couldn't get minute");
        let hour = *map.get("hour").expect("ERROR: Couldn't get hour");
        let day = *map.get("day").expect("ERROR: Couldn't get day");
        let month = *map.get("month").expect("ERROR: Couldn't get month");
        let year = *map.get("year").expect("ERROR: Couldn't get year");

        return MyDateTime { timestamp, second, minute, hour, day, month, year };
    }

    fn parse_timestamp(timestamp: usize) -> HashMap<String, usize> {
        let mut map = HashMap::default();
        let days_since_1970 = timestamp / 86400;

        let second = timestamp % 60;
        let minute = (timestamp / 60) % 60;
        let hour = (timestamp / 60 / 60) % 24;
        let day = timestamp / 60 / 60 / 24;

        let years = (day as f64 / 365.25) as usize;
        let year = 1970 + years;

        let leap_year_count = days_since_1970 / 1461;

        let mut days_this_year = (days_since_1970 - (leap_year_count * 1461)) % 365;

        let mut leap_year = false;
        if year % 4 == 0 && year % 100 == 0 && year % 400 == 0 {
            leap_year = true;
            days_this_year += 1;
        }

        let days_in_month: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut month = 0;
        while days_this_year > days_in_month[month] {
            let mut days_this_month = days_in_month[month];
            if month == 1 && leap_year {
                days_this_month += 1;
            }
            days_this_year -= days_this_month;
            month += 1;
        }
        let day = days_this_year+1;
        map.insert(String::from("hour"), hour);
        map.insert(String::from("minute"), minute);
        map.insert(String::from("second"), second);
        map.insert(String::from("year"), year);
        map.insert(String::from("month"), 1+month);
        map.insert(String::from("day"), day);
        return map;
    }
}

impl Display for MyDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}

impl From<String> for MyDateTime {
    fn from(value: String) -> Self {
        todo!()
    }
}

impl From<&str> for MyDateTime {
    fn from(value: &str) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod mydatetimetests {
    use super::MyDateTime;

    #[test]
    fn str_to_datetime() {
        let input = "1970-01-01T00:00:00";
        let result = MyDateTime::from(input);

        assert_eq!(result.to_string(), input);
    }

    #[test]
    fn string_to_datetime() {
        let input = String::from("1970-01-01T00:00:00");
        let result = MyDateTime::from(input.clone());

        assert_eq!(result.to_string(), input);
    }

    #[test]
    fn mydatetime() {
        let inputs = [1681448931, 1681105217, 1313905026, 0, 1];
        let expected = ["2023-04-14T05:08:51", "2023-04-10T05:40:17",
        "2011-08-21T05:37:06", "1970-01-01T00:00:00", "1970-01-01T00:00:01"];

        for (i, _) in inputs.iter().enumerate() {
            let result = MyDateTime::new(inputs[i]).to_string();
            assert_eq!(result, expected[i]);
        }
    }
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
