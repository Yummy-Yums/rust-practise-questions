// Write a program to store and print the roll no., name,age and marks of a student using structures.

pub mod student_program {
    use once_cell::sync::Lazy;
    use rand::random;
    use std::collections::HashMap;
    use std::sync::Mutex;


    pub struct Student {
        roll_no: u8,
        name: String,
        age: u8,
        marks: u8
    }

    static STORE: Lazy<Mutex<HashMap<u8, Student>>> = Lazy::new(|| Mutex::new(HashMap::new()));

    impl Student {

        pub fn new(name: String, age: u8, marks: u8) -> Student {
            Student {
                roll_no: random::<u8>(),
                name,
                age,
                marks,
            }
        }

        pub fn store_student_record(self) {
            STORE.lock().unwrap().insert(self.roll_no, self);
        }

        pub fn print_student_info() {
            STORE.lock().unwrap().iter().for_each(|(roll_no, student)| {
                println!("Roll no: {}, name: {}", roll_no, student.name);
            })
        }

    }
}