mod solution;

use solution::*;
use std::fs;

const DATA: &[fn(String)] = &[day1::main, day2::main, day3::main, day4::main];
fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("What day you want to solve ?");
    let mut day = String::new();
    std::io::stdin()
        .read_line(&mut day)
        .expect("Unable to read line");
    let day = day.trim().parse::<usize>().unwrap();
    if day < 1 || day > DATA.len() {
        println!("Invalid day");
        return;
    }
    DATA[day as usize - 1](input);
}
