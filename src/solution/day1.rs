fn part_one(input: String) -> String {
    let mut max_val = 0;
    let mut now = 0;
    input.lines().for_each(|line| {
        let line_trim = line.trim();
        if line_trim == "" {
            now = 0;
            return;
        }
        let val = line_trim.parse::<i32>().unwrap();
        now += val;
        if now > max_val {
            max_val = now;
        }
    });

    format!("{}", max_val)
}

fn part_two(input: String) -> String {
    let mut arr: Vec<i32> = Vec::new();
    let mut now = 0;
    input.lines().for_each(|line| {
        let line_trim = line.trim();
        if line_trim == "" {
            arr.push(now);
            now = 0;
            return;
        }
        let val = line_trim.parse::<i32>().unwrap();
        now += val;
    });

    arr.push(now);
    arr.sort_by(|a, b| b.cmp(a));

    format!("{}", arr[0] + arr[1] + arr[2])
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day1_test {
    use super::*;
    use std::fs;

    const SAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        assert_eq!("24000", part_one(SAMPLE.to_string()));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day1.txt");
        assert_eq!("68467", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        assert_eq!("45000", part_two(SAMPLE.to_string()));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day1.txt");
        assert_eq!("203420", part_two(input));
    }
}
