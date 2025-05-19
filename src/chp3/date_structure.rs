/*
    Create a structure named Date having day, month and year as its elements.
    Store the current date in the structure. Now add 45 days to the current date and display the final date.
 */


pub mod date_structure {
    use chrono::Datelike;

    #[derive(Debug, PartialEq)]
    pub struct Date {
        pub year: i32,
        pub month: u32,
        pub day: u32,
    }

    impl Date {

        pub fn from_chrono(date: chrono::NaiveDate) -> Self {
            Date {
                day: date.day(),
                month: date.month(),
                year: date.year()
            }
        }

        pub fn to_chrono(&self) -> chrono::NaiveDate {
            chrono::NaiveDate::from_ymd_opt(self.year, self.month, self.day)
                .expect("Invalid date")
        }

        pub fn display(&self) {
            println!("Final Date: {:02}-{:02}-{}", self.day, self.month, self.year);

        }
    }

    pub fn main() {

    }
}

mod tests {
    use crate::chp3::date_structure::date_structure::*;
    use chrono::{Duration, Local};

    #[test]
    pub fn test_date_structure(){
        // Get current date
        let today = Local::now().date_naive();
        let mut current_date = crate::chp3::date_structure::date_structure::Date::from_chrono(today);

        println!("Current Date: {:02}-{:02}-{}",
                 current_date.day, current_date.month, current_date.year);

        // Add 45 days using chrono's date handling
        let new_date = current_date.to_chrono() + Duration::days(45);
        current_date = crate::chp3::date_structure::date_structure::Date::from_chrono(new_date);

        // Display the final date
        assert_eq!(Date{
            year: 2025,
            month: 07,
            day: 03,
        }, current_date);

    }
}