// Write a program to find prime numbers upto a number N

pub fn find_prime_numbers(limit: i64) -> Vec<i64> {

    if limit < 2 {
        return vec![];
    }

    let mut primes = Vec::new();

    for num in 2..=limit {
        let mut is_prime = true;

        // Check divisibility from 2 to sqrt(num)
        let max_divisor = (num as f64).sqrt() as i64 + 1;

        for divisor in 2..max_divisor {
            if num % divisor == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(num);
        }
    }

    primes

}


mod prime_tests {
    use super::find_prime_numbers;

    #[test]
    pub fn test_find_prime() {

        let result: Vec<Vec<i64>> = vec![
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31],
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23],
            vec![2, 3, 5, 7, 11, 13, 17, 19],
            vec![2, 3, 5, 7, 11, 13]
        ];
        assert_eq!(find_prime_numbers( 32), result[0]);
        assert_eq!(find_prime_numbers( 27), result[1]);
        assert_eq!(find_prime_numbers( 20), result[2]);
        assert_eq!(find_prime_numbers( 13), result[3]);
    }

    #[test]
    pub fn it_returns_wrong_prime() {
        assert_ne!(find_prime_numbers( 1900), vec![1900]);
        assert_ne!(find_prime_numbers( 1950), vec![1900]);
        assert_ne!(find_prime_numbers( 1908), vec![1900]);
        assert_ne!(find_prime_numbers( 2013), vec![1900]);
    }
}