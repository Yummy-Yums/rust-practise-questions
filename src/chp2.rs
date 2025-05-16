use crate::chp2::find_prime::find_prime_numbers;

mod infinite_loop;
mod table;
mod print_pattern;
mod check_odd_or_even;
mod factorial;
mod check_leap_year;
mod swap_numbers;
mod find_prime;

pub fn chp2_main(){

    println!("{:?}",find_prime_numbers(32));
    println!("{:?}",find_prime_numbers(27));
    println!("{:?}",find_prime_numbers(20));
    println!("{:?}",find_prime_numbers(13));
}
