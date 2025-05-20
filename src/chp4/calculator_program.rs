/*
    Create a calculator with the help of an enum named Operation that as values such as Add, Sub, Div, Multi.
    hint: For input like 2+5*10 it should first evaluate 5*10 and then add 2 to it
 */


pub mod calculator_program {
    use lib;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy)]
    enum Operation {
        Add,
        Sub,
        Div,
        Multiply,
        UnknownOperator
    }

    pub fn operation(operation: Operation, a: i8, b: i8) -> Result<i8, String> {
        match operation {
            Operation::Add => Ok(a + b),
            Operation::Sub => Ok(a - b),
            Operation::Div => {
                if b == 0{
                    Err("Division by zero".to_string())
                } else {
                    Ok(a / b)
                }
            },
            Operation::Multiply => Ok(a * b),
            Operation::UnknownOperator => Err("Unknown operator".to_string()),
        }
    }

    pub fn calc_main(input: &str) -> Result<i8, String> {

        let ops_map =
            HashMap::<char, Operation>::from([
                ('+', Operation::Add),
                ('*', Operation::Multiply),
                ('-', Operation::Sub),
                ('/', Operation::Div),
            ]);

        println!("Welcome to Calculator");

        // io::stdout()
        //     .flush()
        //     .unwrap();
        //
        // let input = lib::get_input();

        let mut numbers: Vec<i8> = Vec::new();
        let mut ops: Vec<Operation> = Vec::new();
        let mut current_num: String = String::new();

        for c in input.chars() {
            if c.is_ascii_digit() {
                // push into current_num
                current_num.push(c);
            } else if ops_map.contains_key(&c) {
                // push both number and op at the same time
                if !current_num.is_empty() {
                    numbers.push(current_num.parse::<i8>().unwrap());
                    current_num.clear();
                }

                ops.push(*ops_map.get(&c).unwrap_or(&Operation::UnknownOperator));
            }
        }

        if !current_num.is_empty() {
            numbers.push(current_num.parse::<i8>().unwrap());
        }

        println!("Welcome to Calculator {} {}", ops.len(), numbers.len());

        // Add last number
        if numbers.len() != ops.len() + 1 {
            print!("Error: Mismatched numbers and operators");
            Err("Error: Mismatched numbers and operators")?;
        }

        // Calculate
        let mut result: i8 = numbers[0];
        for (i, &op) in ops.iter().enumerate() {
            match operation(op, result, numbers[i + 1]) {
                Ok(num) => result = num,
                Err(msg) => {
                    println!("Error: {}", msg);


                }
            }
        }

        println!("Result: {}", result);
        Ok(result)

    }
}


mod tests {
    use crate::chp4::calculator_program::calculator_program::calc_main;

    #[test]
    pub fn it_should_pass(){

        assert_eq!(8, calc_main("2+2*2").unwrap());
        assert_eq!(4, calc_main("2*2").unwrap());
        assert_eq!(12, calc_main("2+3*2+2").unwrap());
        assert_eq!(8, calc_main("2+2*2").unwrap());

    }

    #[test]
    #[should_panic]
    pub fn it_should_fail(){
        calc_main("2+2*").unwrap();
    }
}