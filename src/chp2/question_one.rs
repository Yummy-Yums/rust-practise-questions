pub fn infinite_loop(){

    loop {
        println!("Hello World");
    }

}

#[cfg(test)]
mod infinite_loop_tests {
    use std::io;
    use std::io::Stdout;

    #[test]
    pub fn test_infinite_loop() {
        // firgure out how to test this

    }

}