mod chapter3_homework;

use chapter3_homework::parse_temperature;
use crate::chapter3_homework::{fibonacci, print_christmas_song};

fn main() {
    parse_temperature();
    println!("{}", fibonacci(20));
    print_christmas_song();
}
