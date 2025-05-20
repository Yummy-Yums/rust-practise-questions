/*
    Create a Regex to extract dates from this string It was on 2019-03-14,
    almost after a year from 2018-02-11 and store in a Struct with fields of day, month and year.
    hint: Use Regex crate
 */

pub mod date_program {
    use regex::Regex;
    use std::fmt;

    #[derive(Debug, PartialEq)]
    pub struct Date {
        pub day: u32,
        pub month: u32,
        pub year: i32,
    }

    impl fmt::Display for Date {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
        }
    }

    pub fn extract_dates(text: &str) -> Vec<Date> {
        let date_regex = Regex::new(r"\b(\d{4})-(\d{2})-(\d{2})\b").unwrap();
        let mut dates = Vec::new();

        for cap in date_regex.captures_iter(text) {
            let year = cap[1].parse().unwrap();
            let month = cap[2].parse().unwrap();
            let day = cap[3].parse().unwrap();

            dates.push(Date { day, month, year });
        }

        dates
    }
}

mod test{
    use crate::chp4::extract_date_program::date_program::Date;

    #[test]
    fn test_extract_dates() {
        assert_eq!(super::date_program::extract_dates("2010-01-01"), [Date { day: 1, month: 1, year: 2010 }]);
        assert_ne!(super::date_program::extract_dates("2010-01-01"), [Date { day: 2, month: 1, year: 2010 }])
    }
}