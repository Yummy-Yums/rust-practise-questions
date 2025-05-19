/*
    Run an infinite loop to keep on printing Hello, World!.
    hint: you might want to use ctrl+c or ctrl+z to exit the infinite loop.
 */

pub fn infinite_loop(){

    loop {
        println!("Hello World");
    }

}

#[cfg(test)]
mod infinite_loop_tests {
    #[test]
    pub fn test_infinite_loop() {
        // figure out how to test this

    }

}