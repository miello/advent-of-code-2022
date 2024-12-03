fn split_input(input: &str) -> Vec<Vec<&str>> {
    let mut index = 0;
    let mut sz = 0;
    let mut result: Vec<Vec<&str>> = Vec::new();
    let data_line = input.lines();
    for line in data_line {
        if !line.contains("[") {
            let split_data: Vec<&str> = line.trim().split(" ").collect();
            sz = split_data[split_data.len() - 1].parse::<i32>().unwrap();
            break;
        }
        index += 1;
    }

    for _ in 0..sz {
        result.push(Vec::new());
    }

    for i in (0..index).rev() {
        let line = input
            .lines()
            .nth(i)
            .unwrap()
            .split("")
            .collect::<Vec<&str>>();
        let mut index = 0;

        for j in (2..line.len()).step_by(4) {
            if line[j] != " " {
                result[index].push(line[j]);
            }
            index += 1;
        }
    }

    result
}

fn split_command(input: &str) -> (i32, usize, usize) {
    let split_data: Vec<&str> = input.trim().split(" ").collect();
    let sz = split_data[1].parse::<i32>().unwrap();
    let from = split_data[3].parse::<usize>().unwrap();
    let to = split_data[5].parse::<usize>().unwrap();

    (sz, from, to)
}

fn part_one(input: String) -> String {
    let mut stock = split_input(&input);
    input.lines().for_each(|f| {
        if !f.contains("move") {
            return;
        }
        let (sz, from, to) = split_command(f);

        for _ in 0..sz {
            if stock[from - 1].is_empty() {
                break;
            }
            let item = stock[from - 1].pop().unwrap();
            stock[to - 1].push(item);
        }
    });

    stock
        .iter()
        .map(|f| f[f.len() - 1].to_string())
        .collect::<String>()
}

fn part_two(input: String) -> String {
    let mut stock = split_input(&input);
    input.lines().for_each(|f| {
        if !f.contains("move") {
            return;
        }
        let (sz, from, to) = split_command(f);

        let cur_sz = stock[from - 1].len() as i32;
        let mut diff = 0;
        if cur_sz >= sz {
            diff = cur_sz - sz;
        }

        let tmp: Vec<&str> = stock[from - 1].drain((diff as usize)..).collect();
        stock[to - 1].extend(tmp);
    });

    stock
        .iter()
        .map(|f| f[f.len() - 1].to_string())
        .collect::<String>()
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day5_test {
    use super::*;
    use std::fs;

    const SAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    ";

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        assert_eq!("CMZ", part_one(SAMPLE.to_string()));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day5.txt");
        assert_eq!("VWLCWGSDQ", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        assert_eq!("MCD", part_two(SAMPLE.to_string()));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day5.txt");
        assert_eq!("TCGLQSLPW", part_two(input));
    }
}
