/*
    Rust does not allow addition of integer and float. Overload + so that this is possible.
 */

pub mod int_plus_float_program {

    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Number(pub f64);

    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            Number(value as f64)
        }
    }

    impl From<i64> for Number {
        fn from(value: i64) -> Self {
            Number(value as f64)
        }
    }

    impl From<f32> for Number {
        fn from(value: f32) -> Self {
            Number(value as f64)
        }
    }

    impl Add for Number {
        type Output = Number;

        fn add(self, other: Number) -> Self::Output {
            Number(self.0 + other.0)
        }
    }

    impl Add<i32> for Number {
        type Output = Self;

        fn add(self, other: i32) -> Self::Output {
            Number(self.0 + other as f64)
        }
    }

    impl Add<f32> for Number {
        type Output = Self;

        fn add(self, other: f32) -> Self::Output {
            Number(self.0 + other as f64)
        }
    }

    impl Add<Number> for f32 {
        type Output = Number;

        fn add(self, other: Number) -> Self::Output {
            Number(self as f64 + other.0)
        }
    }

    impl Add<Number> for i32 {
        type Output = Number;

        fn add(self, other: Number) -> Self::Output {
            Number(self as f64 + other.0)
        }
    }

}

mod tests{

    use super::int_plus_float_program::*;
    #[test]
    pub fn test_int_plus_float_program(){
        let integer = 5;
        let float: f32 = 3.14;

        let num_from_int   = Number::from(integer);
        let num_from_float = Number::from(float);

        let result1 = num_from_int + num_from_float;
        assert_eq!(Number(8.140000104904175), result1);

        // Or use the direct implementations
        let result2 = num_from_int + 10;
        assert_eq!(Number(15.0), result2);

        let result3 = 10 + num_from_float;
        assert_eq!(Number(13.140000104904175), result3);

        let result4 = num_from_int + 2.5f32;
        assert_eq!(Number(7.5), result4);

        let result5 = 2.5f32 + num_from_float;
        assert_eq!(Number(5.640000104904175), result5);
    }
}