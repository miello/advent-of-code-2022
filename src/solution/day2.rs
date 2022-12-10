fn convert_opponent_to_mine(opponent: &str) -> &str {
    match opponent {
        "A" => "X",
        "B" => "Y",
        "C" => "Z",
        _ => "",
    }
}

fn get_score(mine: &str, opponent: &str) -> i32 {
    let mut current_score = 0;
    match mine {
        "X" => {
            current_score += 1;

            if opponent == "C" {
                current_score += 6;
            }

            if opponent == "A" {
                current_score += 3;
            }
        }
        "Y" => {
            current_score += 2;

            if opponent == "A" {
                current_score += 6;
            }

            if opponent == "B" {
                current_score += 3;
            }
        }
        "Z" => {
            current_score += 3;

            if opponent == "B" {
                current_score += 6;
            }

            if opponent == "C" {
                current_score += 3;
            }
        }
        _ => {}
    };
    return current_score;
}

fn part_one(input: String) -> String {
    let mut current_score = 0;
    for line in input.lines() {
        let mut split = line.split(" ");
        let opponent = split.next().unwrap();
        let mine = split.next().unwrap();
        current_score += get_score(mine, opponent);
    }
    format!("{}", current_score)
}

fn part_two(input: String) -> String {
    let mut current_score = 0;
    for line in input.lines() {
        let mut split = line.split(" ");
        let opponent = split.next().unwrap();
        let result_status = split.next().unwrap();
        let mut mine = "";
        match result_status {
            "X" => match opponent {
                "A" => mine = "C",
                "B" => mine = "A",
                "C" => mine = "B",
                _ => {}
            },
            "Y" => {
                mine = opponent;
            }
            "Z" => match opponent {
                "A" => mine = "B",
                "B" => mine = "C",
                "C" => mine = "A",
                _ => {}
            },
            _ => {}
        };

        current_score += get_score(convert_opponent_to_mine(mine), opponent);
    }

    format!("{}", current_score)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day2_test {
    use super::*;
    use std::fs;

    const SAMPLE: &str = "A Y
B X
C Z";

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        assert_eq!("15", part_one(SAMPLE.to_string()));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day2.txt");
        assert_eq!("12740", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        assert_eq!("12", part_two(SAMPLE.to_string()));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day2.txt");
        assert_eq!("11980", part_two(input));
    }
}
