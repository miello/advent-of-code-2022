struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn is_fully_inside(&self, other: &Point) -> bool {
        self.x <= other.x && self.y >= other.y
    }

    fn is_overlap(&self, other: &Point) -> bool {
        !(self.x > other.y || self.y < other.x)
    }
}

fn split_data(input: String) -> Point {
    println!("{:?}", input);
    let mut split = input.split("-");
    let x = split.next().unwrap().parse::<i32>().unwrap();
    let y = split.next().unwrap().parse::<i32>().unwrap();
    return Point::new(x, y);
}

fn part_one(input: String) -> String {
    let mut fully = 0;
    for line in input.lines() {
        let mut spliting = line.split(',');
        let first_point = split_data(spliting.next().unwrap().to_string());
        let second_point = split_data(spliting.next().unwrap().to_string());

        if second_point.is_fully_inside(&first_point) {
            fully += 1;
            continue;
        }

        if first_point.is_fully_inside(&second_point) {
            fully += 1;
            continue;
        }
    }

    format!("{}", fully)
}

fn part_two(input: String) -> String {
    let mut fully = 0;
    for line in input.lines() {
        let mut spliting = line.split(',');
        let first_point = split_data(spliting.next().unwrap().to_string());
        let second_point = split_data(spliting.next().unwrap().to_string());

        if first_point.is_overlap(&second_point) {
            fully += 1;
        }
    }

    format!("{}", fully)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day4_test {
    use super::*;
    use std::fs;

    const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        assert_eq!("2", part_one(SAMPLE.to_string()));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day4.txt");
        assert_eq!("475", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        assert_eq!("4", part_two(SAMPLE.to_string()));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day4.txt");
        assert_eq!("825", part_two(input));
    }
}
