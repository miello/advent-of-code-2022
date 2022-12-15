mod solution;

use solution::*;
use std::fs;

const DATA: &[fn() -> (fn(String) -> String, fn(String) -> String)] = &[
    day1::main,
    day2::main,
    day3::main,
    day4::main,
    day5::main,
    day6::main,
    day7::main,
    day8::main,
    day9::main,
    day10::main,
    day11::main,
    day12::main,
    day13::main,
    day14::main,
    day15::main,
];

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
    let (part_one, part_two) = DATA[day as usize - 1]();
    println!("Part 1: {}", part_one(input.clone()));
    println!("Part 2: {}", part_two(input.clone()));
}
