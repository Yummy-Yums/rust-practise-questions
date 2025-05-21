/*
    Create a struct named Class that contains the class size, section and grade.
    Overload the >, <, >=, <=, == operators to compare class sizes of various Classes.

    Implement custom Drop for a struct Hero that contains a field name of type string.
    The drop should print "Oh no !!! Our hero {name} is defeated".
    Run the program with just declaring a variable of type Hero.

    Create the struct named World that contains the previous named struct Hero,
    define drop for it so that it prints "The world ends here !!!".
    Observe the order in which World and Hero contained by it are dropped.

    Create a struct named Table that has a generic field legs_info of type T.
    Create a function (not a method) named show that accepts function parameters &Table<Display>
    and displays legs_info. Create 2 variables one that contains T of type String: "Work in progress..."
     and usize: 4. hint: use ?Sized.

 */

pub mod class_program {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fmt::Display;

    #[derive(Debug)]
    pub struct Class {
        pub class_size: usize,
        pub section: HashMap<String, i8>,
        pub grade: i8
    }

    impl PartialEq for Class {
        fn eq(&self, other: &Self) -> bool {
            self.class_size == other.class_size
        }
    }

    impl PartialOrd for Class {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.class_size.partial_cmp(&other.class_size)
        }

        fn lt(&self, other: &Self) -> bool {
            self.class_size < other.class_size
        }

        fn le(&self, other: &Self) -> bool {
            self.class_size <= other.class_size
        }

        fn gt(&self, other: &Self) -> bool {
            self.class_size > other.class_size
        }

        fn ge(&self, other: &Self) -> bool {
            self.class_size >= other.class_size
        }
    }

    struct Hero {
        name: String
    }

    impl Drop for Hero {
        fn drop(&mut self) {
            println!("The world ends here!!!");
            true;
        }
    }

    struct World {
        hello: Hero
    }

    impl Drop for World {
        fn drop(&mut self) {
            println!("The world ends here!!");
            true;
        }
    }

    #[derive(Debug)]
    struct Table<T: ?Sized> {
        legs_info: T
    }

    pub fn show(table: &Table<&dyn Display>){
        println!("{}", table.legs_info)
    }

    pub fn class_main(){
        let er = Hero{name: String::from("wer")};
        let re = World{hello: er};

        let a: Table<&dyn Display> = Table{
            legs_info: &String::from("Works in progress...") as &dyn Display
        };
        let b: Table<&dyn Display> = Table{
            legs_info: &4usize as &dyn Display
        };

        show(&a);
        show(&b);

    }
}

mod tests{
    use std::collections::HashMap;
    use super::class_program::*;

    #[test]
    pub fn test_class_program(){

        let first_class  = Class{class_size: 4, section: HashMap::new(), grade: 1};
        let second_class = Class{class_size: 10, section: HashMap::new(), grade: 3};

        assert!(first_class < second_class);
        assert!(first_class <= second_class);
        assert!(!(first_class > second_class));
        assert!(!(first_class >= second_class));
        assert_ne!(first_class , second_class);

    }

}