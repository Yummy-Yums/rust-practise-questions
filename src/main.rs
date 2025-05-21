mod chp1_basics;
mod chp2;
mod chp3;
mod chp4;
mod chp5;
mod chp6;
mod chp7;

use chp1_basics::chp1_main;
use chp2::chp2_main;

fn main() {
    println!("Hello, world!");
    chp1_main();
    chp2_main();
}
