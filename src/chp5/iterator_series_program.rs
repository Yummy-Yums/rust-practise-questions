/*
    a. Implement custom Iterator trait for a struct named GeometricSeries
    that has 3 fields first_number, current_number and ratio of type i32.
    The iterator should return the next 11 numbers in geometric progression.
    hint: Use .take(11) to get the next 11 in for loop.

    b. Implement the same as above for a FibonacciSeries struct.
 */

pub mod iterator_series_program{

    #[derive(Debug)]
    pub struct GeometricSeries{
        first_number: i32,
        current_number: i32,
        ratio: i32
    }

    impl Iterator for GeometricSeries {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let mut current: i32 = self.first_number;
            self.current_number = current * self.ratio;
            self.first_number = self.current_number;
            Some(current as u32)
        }
    }

    pub fn geometric_progression() -> GeometricSeries {
        GeometricSeries { first_number: 3, current_number: 3, ratio: 2}
    }

    pub struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;
        // a. r
        fn next(&mut self) -> Option<Self::Item> {
            let current = self.curr;

            self.curr = self.next;
            self.next = current + self.next;

            Some(current)
        }
    }

    // Returns a Fibonacci sequence generator
    pub fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }

    pub fn sum<T: Iterator<Item=u32>>(iter: T, index: usize) -> u32{
        let mut sum = 0;

        for i in iter.take(index) {
            sum += i;
        }
        sum
    }

}

mod tests {
    use crate::chp5::iterator_series_program::iterator_series_program::{fibonacci, geometric_progression, sum};

    #[test]
    pub fn fib_tests() {

        assert_eq!(vec![0, 1, 1, 2], fibonacci().into_iter().take(4).collect::<Vec<_>>());
        assert_ne!(vec![0, 1, 1, 2], fibonacci().into_iter().take(2).collect::<Vec<_>>());

        assert_eq!(vec![0, 1, 1, 2, 3], fibonacci().into_iter().take(5).collect::<Vec<_>>());
        assert_ne!(vec![0, 1, 1, 2], fibonacci().into_iter().take(3).collect::<Vec<_>>());

        assert_eq!(7, sum(fibonacci().into_iter(), 5));
        assert_ne!(4, sum(fibonacci().into_iter(), 5));
    }

    #[test]
    pub fn gp_tests() {

        assert_eq!(vec![3, 6, 12, 24], geometric_progression().into_iter().take(4).collect::<Vec<_>>());
        assert_ne!(vec![0, 1, 1, 2], geometric_progression().into_iter().take(2).collect::<Vec<_>>());

        assert_eq!(vec![3, 6, 12, 24, 48], geometric_progression().into_iter().take(5).collect::<Vec<_>>());
        assert_ne!(vec![0, 1, 1, 2], geometric_progression().into_iter().take(3).collect::<Vec<_>>());

        assert_eq!(93, sum(geometric_progression().into_iter(), 5));
        assert_ne!(4, sum(geometric_progression().into_iter(), 5));

    }
}