// Find and print factorial of a program using recursion.

pub fn factorial(n: usize) -> usize {
    if n == 0 || n == 1 {
        1
    } else { n * factorial(n-1) }
}

#[cfg(test)]
mod factorial_tests {
    use super::*;

    #[test]
    pub fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

}