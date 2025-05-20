/*
    Write a generic function name Multiply that multiplies all usize, isize and fsize type values.
 */

pub mod multiplier_program {
    use std::ops::Mul;

    pub fn multiply_usize<T>(values: &[T]) -> T
    where
        T: Mul<T, Output=T> + Copy + From<usize> {
        let mut product = T::from(1);

        for &num in values {
            product = product * num;
        }

        product
    }

    pub fn multiply_isize<T>(values: &[T]) -> T
    where
        T: Mul<T, Output=T> + Copy + From<isize> {
        let mut product = T::from(1);

        for &num in values {
            product = product * num;
        }

        product
    }

    pub fn multiply_floats<T>(values: &[T]) -> T
    where
        T: Mul<T, Output=T> + Copy + From<f32> {
        let mut product = T::from(1.0);

        for &num in values {
            product = product * num;
        }

        product
    }

}

mod tests{
    use super::multiplier_program::*;

    #[test]
    pub fn test_multiply_integers(){
        let nums_usize: [usize; 3] = [2usize, 3, 4];
        let nums_isize: [isize; 3] = [-2isize, 3, 4];

        assert_eq!(24, multiply_usize(&nums_usize));
        assert_eq!(-24, multiply_isize(&nums_isize));
    }

    #[test]
    pub fn test_multiply_floats(){

        let nums_f64: [f64; 3] = [2.5f64, 3.0, 4.0];
        let nums_f32: [f32; 3] = [2.5f32, 3.0, 4.0];

        assert_eq!(30.0, multiply_floats(&nums_f32));
        assert_eq!(30.0, multiply_floats(&nums_f64));
    }
}