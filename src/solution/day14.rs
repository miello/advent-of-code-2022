use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
enum Space {
    Air,
    Rock,
    Sand,
}

fn generate_wall(input: String) -> (Vec<Vec<Space>>, i32) {
    let mut table = Vec::new();
    let mut row = Vec::new();

    row.resize(5000, Space::Air);
    table.resize(1000, row);
    let mut max_x = 0;

    for line in input.lines() {
        let wall_arr = line
            .split(" -> ")
            .map(|f| {
                f.split(",")
                    .map(|f| f.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        let len = wall_arr.len();

        for idx in 1..len {
            let mut x_pos = (wall_arr[idx][1], wall_arr[idx][0]);
            let y_pos = (wall_arr[idx - 1][1], wall_arr[idx - 1][0]);

            let dx = match x_pos.0.cmp(&y_pos.0) {
                Ordering::Equal => 0,
                Ordering::Greater => -1,
                Ordering::Less => 1,
            };

            let dy = match x_pos.1.cmp(&y_pos.1) {
                Ordering::Equal => 0,
                Ordering::Greater => -1,
                Ordering::Less => 1,
            };
            max_x = max_x.max(x_pos.0);
            max_x = max_x.max(y_pos.0);

            while x_pos != y_pos {
                table[x_pos.0 as usize][x_pos.1 as usize] = Space::Rock;
                x_pos.0 += dx;
                x_pos.1 += dy;
            }
            table[y_pos.0 as usize][y_pos.1 as usize] = Space::Rock;
        }
    }

    (table, max_x)
}

fn get_next_position(table: &Vec<Vec<Space>>) -> (bool, (i32, i32)) {
    let mut x = 0;
    let mut y = 500;

    loop {
        if x == 999 {
            break;
        } else if table[x + 1][y] == Space::Air {
            x += 1;
        } else if table[x + 1][y - 1] == Space::Air {
            x += 1;
            y -= 1;
        } else if table[x + 1][y + 1] == Space::Air {
            x += 1;
            y += 1;
        } else {
            return (false, (x as i32, y as i32));
        }
    }

    (true, (0, 0))
}

fn add_extra_floor(table: &mut Vec<Vec<Space>>, max_x: usize) {
    for i in 0..5000 {
        table[max_x + 2][i] = Space::Rock;
    }
}

fn part_one(input: String) -> String {
    let (mut table, _) = generate_wall(input);
    let mut cnt = 0;
    loop {
        let (is_void, (x, y)) = get_next_position(&table);
        if is_void {
            break;
        }
        cnt += 1;
        table[x as usize][y as usize] = Space::Sand;
    }
    format!("{}", cnt)
}

fn part_two(input: String) -> String {
    let (mut table, max_x) = generate_wall(input);
    add_extra_floor(&mut table, max_x as usize);
    let mut cnt = 0;
    loop {
        let (_, (x, y)) = get_next_position(&table);
        cnt += 1;
        if x == 0 && y == 500 {
            break;
        }
        table[x as usize][y as usize] = Space::Sand;
    }
    format!("{}", cnt)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day14_test {
    use super::*;
    use std::fs;

    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        assert_eq!("24", part_one(SAMPLE.to_string()));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day14.txt");
        assert_eq!("737", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        assert_eq!("93", part_two(SAMPLE.to_string()));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day14.txt");
        assert_eq!("28145", part_two(input));
    }
}
