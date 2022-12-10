use std::{cmp::Ordering, collections::HashSet};

fn euclidean_distance(x: (i32, i32), y: (i32, i32)) -> f32 {
    let dx = x.0 as f32 - y.0 as f32;
    let dy = x.1 as f32 - y.1 as f32;
    f32::sqrt(dx * dx + dy * dy)
}

fn get_new_pos(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    let dx = match i32::cmp(&tail.0, &head.0) {
        Ordering::Equal => 0,
        Ordering::Greater => -1,
        Ordering::Less => 1,
    };

    let dy = match i32::cmp(&tail.1, &head.1) {
        Ordering::Equal => 0,
        Ordering::Greater => -1,
        Ordering::Less => 1,
    };

    (tail.0 + dx, tail.1 + dy)
}

fn calculate_distinct_tail(sz: usize) -> impl for<'r> FnMut(Vec<&'r str>) -> usize {
    let mut tail_pos: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = Vec::new();
    for _ in 0..sz {
        rope.push((0, 0));
    }
    move |f: Vec<&str>| {
        let walk = f[1].parse::<usize>().unwrap();
        let mut dx = 0;
        let mut dy = 0;
        match f[0] {
            "L" => dx = -1,
            "R" => dx = 1,
            "U" => dy = 1,
            "D" => dy = -1,
            _ => {}
        }
        for _ in 0..walk {
            rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
            for idx in 1..sz {
                if euclidean_distance(rope[idx], rope[idx - 1]) > f32::sqrt(2.0) {
                    rope[idx] = get_new_pos(rope[idx], rope[idx - 1]);
                }
            }
            tail_pos.insert(rope[rope.len() - 1]);
        }

        tail_pos.len()
    }
}

fn solve(input: String, sz: usize) -> usize {
    let mut solution = 0;
    let mut calculate_fn = calculate_distinct_tail(sz);
    input
        .lines()
        .map(|f| f.split(" ").collect::<Vec<&str>>())
        .for_each(|f| {
            solution = calculate_fn(f);
        });
    solution
}

fn part_one(input: String) -> String {
    format!("{}", solve(input, 2))
}

fn part_two(input: String) -> String {
    format!("{}", solve(input, 10))
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day9_test {
    use super::*;
    use std::fs;

    const SAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const LARGE_SAMPLE: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        assert_eq!("13", part_one(SAMPLE.to_string()));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day9.txt");
        assert_eq!("6236", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        assert_eq!("1", part_two(SAMPLE.to_string()));
        assert_eq!("36", part_two(LARGE_SAMPLE.to_string()));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day9.txt");
        assert_eq!("2449", part_two(input));
    }
}
