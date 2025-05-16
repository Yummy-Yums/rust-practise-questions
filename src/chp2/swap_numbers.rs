pub fn swap_numbers(a: i64, b: i64) -> (i64, i64) {
    let (mut a, mut b) = (a, b);

    println!("Before: a = {}, b = {}", a, b);

    (a, b) = (b, a);  // Swap in one line without a temp variable

    (a, b)
}

mod swap_number_test {
    use super::swap_numbers;

    #[test]
    pub fn test_check_leap_year() {
        assert_eq!(swap_numbers(1, 9), (9, 1));
        assert_eq!(swap_numbers(5, 0), (0, 5));
        assert_eq!(swap_numbers(9, 8), (8, 9));
        assert_eq!(swap_numbers(2, 3), (3, 2));

        assert_ne!(swap_numbers(1, 9), (1, 9));
        assert_ne!(swap_numbers(5, 0), (5, 0));
        assert_ne!(swap_numbers(9, 8), (9, 8));
        assert_ne!(swap_numbers(2, 3), (2, 3));
    }
}