use std::collections::BTreeSet;

const CHAR_LIST: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn cal_score(input: &str) -> i32 {
    return CHAR_LIST.find(input).unwrap().try_into().unwrap();
}

fn part1_solve(input: String) {
    let mut score = 0;
    for line in input.split("\r\n") {
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
    println!("{}", score);
}

fn part2_solve(input: String) {
    let mut score = 0;
    let mut cnt = 0;
    let mut cur_set: BTreeSet<&str> = BTreeSet::new();
    for line in input.split("\r\n") {
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
    println!("{}", score);
}

pub fn main(input: String) {
    println!("Part 1");
    part1_solve(input.clone());
    println!("Part 2");
    part2_solve(input);
}
