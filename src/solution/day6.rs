use std::collections::HashSet;

fn find_distinct_n(input: &str, val: usize) -> usize {
    let char_set = input.chars().collect::<Vec<char>>();
    let mut result = 0;
    for i in 0..(char_set.len() - val) {
        let set = char_set[i..i + val].iter().collect::<HashSet<&char>>();
        if set.len() == val {
            result = i + val;
            break;
        }
    }

    result
}

fn part1_solve(input: String) {
    println!("{}", find_distinct_n(&input, 4));
}

fn part2_solve(input: String) {
    println!("{}", find_distinct_n(&input, 14));
}

pub fn main(input: String) {
    println!("Part 1");
    part1_solve(input.clone());
    println!("Part 2");
    part2_solve(input);
}
