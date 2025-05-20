/*


    Derive a debug trait to print info about your struct that contains name, c1ass and roll.

    Create a generic function to get min of 2 values.
    hint: You might need to use Ord trait bound.

    Implement custom Drop trait for a struct name Student that contains your name, age and roll number.
    It should return Roll number <roll number> has name <name> with age <age> and is a <junior/senior>.
     Being Junior or Senior depends on age (18 or above).

 */


pub mod trait_program {
    use std::fmt;
    use std::fmt::{format, Debug};
    use std::ptr::write;

    #[derive(Debug)]
    struct PrintInfo{
        name : String,
        class : String,
        roll: i8
    }

    pub fn compare<T: Ord>(a: T, b: T) -> T{
        a.min(b)
    }

    pub struct Student{
        pub name : String,
        pub age : i8,
        pub roll_number: i8
    }

    impl Student {
        fn new(name: String, age: i8, roll_number: i8)-> Student {
            Student{
                name,
                age,
                roll_number
            }
        }
    }

    impl Drop for Student {
        fn drop(&mut self) {
            println!(
                "Roll number {} has name {} with age {} and is a {}",
                self.roll_number,
                self.name,
                self.age,
                if self.age > 18 {"senior"} else  {"junior"}
            );
        }
    }

    impl fmt::Debug for Student {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} - {} == {}", self.name, self.age, self.roll_number)
        }
    }


    pub trait Hello {
        fn hello(&self) -> String
        where Self: fmt::Display;
    }

    impl<T: fmt::Display> Hello for T {
        fn hello(&self) -> String {
            println!("Hello, {}", self);
            format(format_args!("Hello, {}!", self))
        }
    }

}

mod tests {
    use crate::chp5::trait_program::trait_program::compare;
    use crate::chp5::trait_program::trait_program::Hello;

    use super::trait_program::*;


    #[test]
    pub fn test_hello(){
        assert_ne!("Hello, Alice", "World".hello());
        assert_eq!("Hello, World!", "World".hello());

    }

    #[test]
    pub fn test_student_info(){
        assert_eq!(3, compare(5, 3));
        assert_eq!(2, compare(5, 2));
        assert_ne!(21, compare(5, 2));
        assert_ne!(23, compare(5, 2));
    }
}