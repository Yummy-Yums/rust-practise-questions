pub mod adder_program {
    use std::ops::Add;

    struct Adder<T: Add<Output = T> + Copy> {
        value: T,
    }

    impl <T: Add<Output = T> + Copy> Adder<T> {
        fn new(value: T) -> Self {
            Adder { value }
        }

        fn add (&mut self, other: T){
            self.value = self.value + other;
        }

        fn get_value(&self) -> &T {
            &self.value
        }
    }

    pub fn adder_main() {
        // Works with integers
        let mut int_adder = Adder::new(5);
        int_adder.add(10);
        println!("Integer sum: {}", int_adder.get_value()); // 15

        // Works with floats
        let mut float_adder = Adder::new(3.5);
        float_adder.add(2.1);
        println!("Float sum: {}", float_adder.get_value()); // 5.6
    }
}