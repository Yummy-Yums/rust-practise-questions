use std::io;


/*
    Take a integer input from the user and print table for that integer using a loop.
 */

pub fn table(){
    let mut input = String::new();

    println!("Please enter a number:");

    io::stdin().read_line(&mut input);
    println!("printing the line \n");

    let parsed_num: i16 = input.trim().parse().expect("expects a number");

    for i in 1..=12 {
        let res = parsed_num * i;

        println!("{} x {} = {}", parsed_num, i, res);
    }

}

#[cfg(test)]
mod table_tests {

    use super::*;

    #[test]
    pub fn test_table() {
        assert!(true, "{:?}", table());
    }

}