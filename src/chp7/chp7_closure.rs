/*
    Implement a macro named addition to add any amount of numbers.
    Eg: addition!(5, 6, 57 ,56, 1) should return 125 and addition!(4, 9) should return 11.
 */

pub mod chp7 {

   #[macro_export]
   macro_rules! addition {
        ($($num:expr),* $(,)?) => {

            {
                let mut result = 0;
                $(
                    result += $num as i32;
                )*
                result
            }
        };
    }

    pub fn accept_closure(f: fn(i32) -> i32) -> i32 {
        f(2)
    }

}

#[cfg(test)]
mod chp7_tests {
    use crate::addition;
    use super::chp7::*;

    #[test]
    fn test_accept_closure(){
        let closure = |x| x * 1;
        assert_eq!(accept_closure(closure), 2);
    }

    fn test_addition_macro(){
        assert_eq!(5, addition!(2, 3));
    }

}

