// Check whether the input number is odd or even and print odd or even respectively.

pub fn check_odd_or_even(input: i64) -> String {
    if input % 2 == 0 {
        println!("even");
        "even".to_string()
    } else if input % 3 == 0 {
        println!("odd");
        "odd".to_string()
    } else {
        println!("odd");
        "odd".to_string()
    }
}

#[cfg(test)]
mod odd_even_tests {
    use super::*;

    #[test]
    pub fn it_returns_even() {
        assert_eq!(check_odd_or_even(0), String::from("even"));
        assert_eq!(check_odd_or_even(2), String::from("even"));
        assert_eq!(check_odd_or_even(4), String::from("even"));
        assert_eq!(check_odd_or_even(58), String::from("even"));
        assert_eq!(check_odd_or_even(72), String::from("even"));
    }

    #[test]
    pub fn it_returns_odd() {
        assert_eq!(check_odd_or_even(1), String::from("odd"));
        assert_eq!(check_odd_or_even(3), String::from("odd"));
        assert_eq!(check_odd_or_even(9), String::from("odd"));
        assert_eq!(check_odd_or_even(15), String::from("odd"));
        assert_eq!(check_odd_or_even(73), String::from("odd"));
    }

    #[test]
    pub fn it_suppose_to_fail() {
        assert_ne!(check_odd_or_even(2), String::from("odd"));
        assert_ne!(check_odd_or_even(3), String::from("even"));
        assert_ne!(check_odd_or_even(9), String::from("even"));
        assert_ne!(check_odd_or_even(16), String::from("odd"));
        assert_ne!(check_odd_or_even(74), String::from("odd"));
    }

}