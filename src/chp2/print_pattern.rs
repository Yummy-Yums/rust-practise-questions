/* Print the following pattern using loops.
         *
        ***
       *****
      *******
     *********
 */

pub fn print_pattern(input: usize) -> String {

    let mut counter = input;
    let mut start = 1;

    let mut res = String::new();

    while counter > 0 {

        let stars = "*".repeat(start);

        let padded_result = format!("{:^width$}", stars, width = 2 * input - 1);
        res.push_str(&padded_result);
        println!("{}", padded_result);

        start = start + 2;
        counter = counter - 1;
    }

    res

}

#[cfg(test)]
mod pattern_tests {
    use super::*;

    #[test]
    pub fn it_returns_empty_string() {
        assert_eq!(print_pattern(0), String::from(""));
    }

    #[test]
    pub fn it_returns_one_asterisk() {
        assert_eq!(print_pattern(1), String::from("*"));
    }

    #[test]
    pub fn it_returns_three_asterisk() {
        let res: String =  "  *   *** *****".to_string();
        assert_eq!(print_pattern(3), res);
    }

}