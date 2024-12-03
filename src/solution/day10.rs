use std::collections::VecDeque;

const MEASURE_CYCLE: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn part_one(input: String) -> String {
    let mut instruction_queue: VecDeque<i32> = VecDeque::new();
    let mut x = 1;
    let mut answer = 0;
    let mut cycle = 0;
    for line in input.lines() {
        cycle += 1;
        let mut split_line = line.split(" ");
        let op = split_line.next().unwrap();
        if MEASURE_CYCLE.contains(&cycle) {
            answer += cycle * x;
        }
        if op.contains("add") {
            let val = split_line.next().unwrap().parse::<i32>().unwrap();
            cycle += 1;
            if MEASURE_CYCLE.contains(&cycle) {
                answer += cycle * x;
            }
            x += val;
            instruction_queue.pop_front();
        }
    }

    format!("{}", answer)
}

fn part_two(input: String) -> String {
    let mut crt_result: Vec<Vec<&str>> = Vec::new();
    crt_result.push(Vec::new());

    let mut x: i32 = 1;
    let mut crt_y: i32 = 0;
    let mut crt_x: i32 = 0;
    for line in input.lines() {
        match crt_y >= x - 1 && crt_y <= x + 1 {
            true => crt_result[crt_x as usize].push("#"),
            false => crt_result[crt_x as usize].push("."),
        };
        crt_y += 1;
        if crt_y == 40 {
            crt_y = 0;
            crt_x += 1;
            crt_result.push(Vec::new());
        }

        let mut split_line = line.split(" ");
        let op = split_line.next().unwrap();

        if op.contains("add") {
            let val = split_line.next().unwrap().parse::<i32>().unwrap();
            match crt_y >= x - 1 && crt_y <= x + 1 {
                true => crt_result[crt_x as usize].push("#"),
                false => crt_result[crt_x as usize].push("."),
            };
            crt_y += 1;
            if crt_y == 40 {
                crt_y = 0;
                crt_x += 1;
                crt_result.push(Vec::new());
            }

            x += val;
        }
    }

    crt_result
        .iter()
        .map(|f| f.join(""))
        .collect::<Vec<String>>()
        .join("\n")
        .trim()
        .to_string()
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day10_test {
    use super::*;
    use std::fs;

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        let input = read_testcase("testcase/day10_sample.txt");
        assert_eq!("13140", part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day10.txt");
        assert_eq!("16880", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let sample_output: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        let input = read_testcase("testcase/day10_sample.txt");
        assert_eq!(sample_output, part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let real_output: &str = "###..#..#..##..####..##....##.###..###..
#..#.#.#..#..#....#.#..#....#.#..#.#..#.
#..#.##...#..#...#..#..#....#.###..#..#.
###..#.#..####..#...####....#.#..#.###..
#.#..#.#..#..#.#....#..#.#..#.#..#.#.#..
#..#.#..#.#..#.####.#..#..##..###..#..#.";
        let input = read_testcase("testcase/day10.txt");
        assert_eq!(real_output, part_two(input));
    }
}
