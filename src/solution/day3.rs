use std::collections::BTreeSet;

const CHAR_LIST: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn cal_score(input: &str) -> i32 {
    return CHAR_LIST.find(input).unwrap().try_into().unwrap();
}

fn part_one(input: String) -> String {
    let mut score = 0;
    for line in input.lines() {
        let sz = line.len();
        let mut set_a = BTreeSet::new();
        let mut set_b = BTreeSet::new();
        line[0..sz / 2].trim().split("").for_each(|f: &str| {
            if f == "" {
                return;
            }
            set_a.insert(f);
        });

        line[sz / 2..sz].trim().split("").for_each(|f: &str| {
            if f == "" {
                return;
            }
            set_b.insert(f);
        });

        set_a.intersection(&set_b).for_each(|ch| {
            score += cal_score(ch) + 1;
        });
    }
    format!("{}", score)
}

fn part_two(input: String) -> String {
    let mut score = 0;
    let mut cnt = 0;
    let mut cur_set: BTreeSet<&str> = BTreeSet::new();
    for line in input.lines() {
        if cnt == 3 {
            cur_set.iter().for_each(|f| {
                score += cal_score(f) + 1;
            });
            cur_set.clear();
            cnt = 0;
        }

        if cnt == 0 {
            line.trim().split("").for_each(|f: &str| {
                if f == "" {
                    return;
                }
                cur_set.insert(f);
            });
        } else {
            let mut new_set = BTreeSet::new();
            line.trim().split("").for_each(|f: &str| {
                if f == "" {
                    return;
                }
                new_set.insert(f);
            });

            cur_set = cur_set.intersection(&new_set).cloned().collect();
        }

        cnt += 1;
    }
    cur_set.iter().for_each(|f| {
        score += cal_score(f) + 1;
    });

    format!("{}", score)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day3_test {
    use super::*;
    use std::fs;

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        assert_eq!("157", part_one(SAMPLE.to_string()));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day3.txt");
        assert_eq!("7446", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        assert_eq!("70", part_two(SAMPLE.to_string()));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day3.txt");
        assert_eq!("2646", part_two(input));
    }
}
