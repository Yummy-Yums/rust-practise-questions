/*
    Create a program to check for leap year.
    note: 1900 is not a leap year.
 */

pub fn check_leap_year(year: usize) -> bool {
    if year == 1900 {
        println!("not leap year");
        false;
    }

    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        println!("a leap year");
        true
    } else {
        println!("not leap year");
        false
    }
}

mod leap_year_test {
    use super::check_leap_year;

    #[test]
    pub fn test_check_leap_year() {
        assert_eq!(check_leap_year(1900), false);
        assert_eq!(check_leap_year(1950), false);
        assert_eq!(check_leap_year(1908), true);
        assert_eq!(check_leap_year(2013), false);
    }
}