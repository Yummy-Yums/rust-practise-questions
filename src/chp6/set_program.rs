/*

Create a struct name Set that contains a Vector of chars and overload the minus operator so that
when 2 structs subtract it removes the chars of 2nd from 1st one.

Create a struct named ComplexNumber that has 2 variables re & im and overload minus
and plus operator to add and subtract complex number.

Overload the ! operator to conjugate the complex number !ComplexNumber and == and != for comparison.

 */

pub mod set_program {
    use std::collections::HashSet;
    use std::ops::*;

    #[derive(Debug, PartialEq)]
    pub struct Set {
        pub x: Vec<char>
    }

    impl Sub for Set {
        type Output = Set;

        fn sub(self, rhs: Self) -> Self::Output {
            let x: HashSet<_> = self.x.into_iter().collect();
            let y: HashSet<_> = rhs.x.into_iter().collect();

            Set{
                x: x.difference(&y).cloned().collect()
            }

        }
    }

    #[derive(Debug)]
    pub struct Complex {
        re: f64,  // Real part
        im: f64,  // Imaginary part
    }

    impl Add for Complex {
        type Output = Complex;

        fn add(self, rhs: Self) -> Self::Output {
            Complex {
                re: self.re + rhs.re,
                im: self.im + rhs.im,
            }
        }
    }

    impl Sub for Complex{
        type Output = Complex;

        fn sub(self, rhs: Self) -> Self::Output {
            Complex {
                re: self.re - rhs.re,
                im: self.im - rhs.im,
            }
        }
    }

    impl PartialEq for Complex{

        fn eq(&self, other: &Self) -> bool {
            self.im == other.im && self.re == self.re
        }

        fn ne(&self, other: &Self) -> bool {
            !self.eq(other)
        }
    }

    impl Not for Complex {
        type Output = bool;

        fn not(self) -> Self::Output {
            self.re == 0.0 && self.im == 0.0
        }
    }

    impl From<(isize, isize)> for Complex{
        fn from(value: (isize, isize)) -> Self {
            Complex {
                re: value.0 as f64,
                im: value.1 as f64,
            }
        }
    }

    pub fn modulo<T: Into<Complex>>(input: T) -> f64{
        let complex = input.into();
        (complex.re.powi(2) + complex.im.powi(2)).sqrt().round()
    }

}

mod tests {

    use std::ops::Sub;
    use super::set_program::*;

    #[test]
    pub fn test_set_program(){

        let a = Set { x: vec!['a', 'b', 'c']};
        let b = Set { x: vec!['b', 'c', 'd']};

        let res = Set { x: vec!['a']};
        assert_eq!(res, a.sub(b))

    }

    #[test]
    pub fn test_complex_program(){

        let a: Complex = (2, 3).into();
        let b: Complex = Complex::from((4, 5));

        assert_eq!(6.0, modulo(b));
        assert_eq!(4.0, modulo(a))

    }

}