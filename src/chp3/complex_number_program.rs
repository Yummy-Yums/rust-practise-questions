/*
    Write a program to compare two dates entered by user. Make a structure named
    Date to store the elements day, month and year to store the dates.
    If the dates are equal, display "Dates are equal" otherwise display "Dates are not equal".
 */

pub mod complex_number_program {

    #[derive(Debug, PartialEq)]
    pub struct Complex {
        pub re: f64,  // Real part
        pub im: f64,  // Imaginary part
    }

    impl Complex {
        pub fn add(&self, other: &Complex) -> Complex {
            Complex {
                re: self.re + other.re,
                im: self.im + other.im,
            }
        }

        pub fn subtract(&self, other: &Complex) -> Complex {
            Complex {
                re: self.re - other.re,
                im: self.im - other.im,
            }
        }

        pub fn multiply(&self, other: &Complex) -> Complex {
            Complex {
                re: self.re * other.re,
                im: self.im * other.im,
            }
        }


    }

}

mod tests {

    use super::complex_number_program::*;

    #[test]
    fn test_complex_number_operations() {
        let z1 = Complex { re: 3.0, im: 2.0 };
        let z2 = Complex { re: 1.0, im: 4.0 };

        let sum        = z1.add(&z2);
        let subtracted = z1.subtract(&z2);
        let multiplied = z1.multiply(&z2);

        assert_eq!(sum, Complex { re: 4.0, im: 6.0 });
        assert_eq!(subtracted, Complex { re: 2.0, im: -2.0 });
        assert_eq!(multiplied, Complex { re: 3.0, im: 8.0 });

    }

}
