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

fn part_one(input: String) -> String {
    format!("{}", find_distinct_n(&input, 4))
}

fn part_two(input: String) -> String {
    format!("{}", find_distinct_n(&input, 14))
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day6_test {
    use super::*;
    use std::fs;

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        assert_eq!("7", part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()));
        assert_eq!("5", part_one("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()));
        assert_eq!("6", part_one("nppdvjthqldpwncqszvftbrmjlhg".to_string()));
        assert_eq!(
            "10",
            part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string())
        );
        assert_eq!(
            "11",
            part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string())
        );
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day6.txt");
        assert_eq!("1356", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        assert_eq!("19", part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()));
        assert_eq!("23", part_two("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()));
        assert_eq!("23", part_two("nppdvjthqldpwncqszvftbrmjlhg".to_string()));
        assert_eq!(
            "29",
            part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string())
        );
        assert_eq!(
            "26",
            part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string())
        );
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day6.txt");
        assert_eq!("2564", part_two(input));
    }
}
