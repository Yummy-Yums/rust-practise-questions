pub mod adder_program {
    use std::ops::Add;

    #[derive(PartialEq)]
    pub struct Adder<T: Add<Output = T> + Copy> {
        value: T,
    }

    impl <T: Add<Output = T> + Copy> Adder<T> {
        pub fn new(value: T) -> Self {
            Adder { value }
        }

        pub fn add (&mut self, other: T){
            self.value = self.value + other;
        }

        pub fn get_value(&self) -> &T {
            &self.value
        }
    }

    pub fn adder_main() {
        // Works with integers
         // 5.6
    }
}

mod test {

    #[test]
    pub fn test_adder() {

        let mut int_adder = crate::chp3::adder_program::adder_program::Adder::new(5);
        int_adder.add(10);

        // Works with floats
        let mut float_adder = crate::chp3::adder_program::adder_program::Adder::new(3.5);
        float_adder.add(2.1);

        assert_eq!(15, *int_adder.get_value());
        assert_eq!(5.6, *float_adder.get_value());

    }

}