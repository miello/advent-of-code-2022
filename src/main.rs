mod solution;

use solution::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("What day you want to solve ?");
    let mut day = String::new();
    std::io::stdin()
        .read_line(&mut day)
        .expect("Unable to read line");
    let day = day.trim().parse::<i32>().unwrap();
    match day {
        1 => day1::main(input),
        2 => day2::main(input),
        3 => day3::main(input),
        _ => println!("Day {} not implemented yet", day),
    };
}
