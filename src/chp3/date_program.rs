/*
    Write a program to compare two dates entered by user.
    Make a structure named Date to store the elements day, month and year to store the dates.
    If the dates are equal, display "Dates are equal" otherwise display "Dates are not equal"
 */

pub mod date_program {
    /*
        - Ask for User to enter date
        - validate dates
    */
    use once_cell::sync::Lazy;
    use regex::Regex;
    use std::cmp::PartialEq;
    use std::collections::HashMap;
    use std::io::{self, BufRead, Write};
    use std::str::FromStr;
    use std::sync::Mutex;

    #[derive(Debug)]
    struct ParseError {
        message: String,
    }

    // Implement the From trait to convert from &str
    impl From<&str> for ParseError {
        fn from(message: &str) -> Self {
            ParseError {
                message: message.to_string(),
            }
        }
    }

    #[derive(Debug)]
    struct Date {
        year: i16,
        month: i8,
        day: i8,
    }

    impl PartialEq for Date {
        fn eq(&self, other: &Self) -> bool {
            self.day  == other.day &&
                self.month == other.month &&
                self.year  == other.year
        }
    }

    impl Date {
        fn new(day: i8, month: i8, year: i16) -> Option<Date> {

            if is_valid_date(day, month, year){
                Some(Date {
                    year,
                    month,
                    day
                })
            } else {
                None
            }

        }

        fn compare() -> bool {
            let mut store =  &mut STORE.lock().unwrap();
            let first_date = store.get(&1).unwrap();
            let second_date = store.get(&2).unwrap();

            first_date == second_date
        }

        fn insert_date(first_date: Date, second_date: Date) {
            let mut store =  &mut STORE.lock().unwrap();
            store.insert(1, first_date);
            store.insert(2, second_date);
        }

        fn print_date() {
            STORE.lock().unwrap().iter().for_each(|item| {
                println!("{:?}", item);
            })
        }
    }

    impl FromStr for Date {

        type Err = ParseError;

        fn from_str(date: &str) -> Result<Self, Self::Err> {
            let dates = date.split("-").collect::<Vec<&str>>();

            match dates[..] {
                [year, month, day] => {
                    let y = year.parse::<i16>().map_err(|_| ParseError::from("Invalid year"))?;
                    let m = month.parse::<i8>().map_err(|_| ParseError::from("Invalid month"))?;
                    let d = day.parse::<i8>().map_err(|_| ParseError::from("Invalid day"))?;

                    if !is_valid_date(d, m, y) {
                        return Err(ParseError::from("Date values out of range"));
                    }

                    Ok(Date::new(d, m, y).unwrap())
                }
                _ => Err(ParseError::from("Invalid date format"))
            }
        }
    }

    fn validate_date(first_date: &str, second_date: &str) -> bool {
        let date_regex = Regex::new(r"^\d{2,4}-\d{1,2}-\d{1,2}$").unwrap();
        date_regex.is_match(first_date) && date_regex.is_match(second_date)
    }

    fn is_valid_date(day: i8, month: i8, year: i16) -> bool {
        if month < 1 || month > 12 || day < 1 {
            return false;
        }

        let days_in_month = match month {
            4 | 6 | 9 | 11 => 30,
            2 => if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 },
            _ => 31
        };

        day <= days_in_month
    }

    static STORE: Lazy<Mutex<HashMap<u8, Date>>> = Lazy::new(|| Mutex::new(HashMap::new()));

    pub fn date_main() -> Result<(), String>{

        loop {

            println!("\n==Welcome to Date Compare==");
            println!("Please enter dates");
            io::stdout().flush().unwrap();

            let mut dates = String::new();
            io::stdin()
                .read_line(&mut dates)
                .unwrap();

            let mut date_older = dates.as_str().trim().split_whitespace().collect::<Vec<&str>>();

            if date_older.is_empty() {
                println!("Date is empty");
                continue
            } else if date_older.len() != 2 {
                println!("Enter 2 dates and not {}", date_older.len());
                continue
            }

            let first_date  = date_older[0];
            let second_date = date_older[1];

            match validate_date(first_date, second_date)  {
                true => {

                    let first = first_date.parse::<Date>().unwrap();
                    let second= second_date.parse::<Date>().unwrap();

                    Date::insert_date(first, second);
                }
                false => {
                    println!("Something is wrong with dates, please check whether it's in's proper format");
                    continue;
                }
            };

            Date::print_date();
            if Date::compare() {
                println!("Dates are equal")
            } else {
                println!("Date are not equal")
            }
            break;
        }

        Ok(())

    }

    pub fn date_main_with_input<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> Result<(), String> {
        loop {
            writeln!(output, "\n==Welcome to Date Compare==").unwrap();
            writeln!(output, "Please enter dates").unwrap();
            output.flush().unwrap();

            let mut dates = String::new();
            input.read_line(&mut dates).unwrap();

            let date_older = dates.trim().split_whitespace().collect::<Vec<&str>>();

            if date_older.is_empty() {
                writeln!(output, "Date is empty").unwrap();
                continue;
            } else if date_older.len() != 2 {
                writeln!(output, "Enter 2 dates and not {}", date_older.len()).unwrap();
                continue;
            }

            let first_date = date_older[0];
            let second_date = date_older[1];

            match validate_date(first_date, second_date) {
                true => {
                    let first = first_date.parse::<Date>().unwrap();
                    let second = second_date.parse::<Date>().unwrap();
                    Date::insert_date(first, second);
                }
                false => {
                    writeln!(
                        output,
                        "Something is wrong with dates, please check whether it's in proper format"
                    )
                        .unwrap();
                    continue;
                }
            }

            Date::print_date();
            if Date::compare() {
                writeln!(output, "Dates are equal").unwrap();
            } else {
                writeln!(output, "Dates are not equal").unwrap();
            }
            break;
        }

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use crate::chp3::date_program::date_program::date_main_with_input;

    #[test]
    fn test_date_main_success() {
        // Simulate input "2023-01-01 2023-01-02"
        let mut input = "2023-01-01 2023-01-02\n".as_bytes();
        let mut output = Vec::new();

        // Assume `validate_date` returns true for these inputs
        assert!(date_main_with_input(&mut input, &mut output).is_ok());

        // Check if output contains expected messages
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Welcome to Date Compare"));
        assert!(output_str.contains("Dates are not equal")); // or "equal" depending on comparison
    }
}
