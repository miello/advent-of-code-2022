use std::collections::VecDeque;

const MOVE_DIFF: &[(i32, i32)] = &[(0, 1), (1, 0), (-1, 0), (0, -1)];

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    fn new_value(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn parse_map(input: String) -> (Vec<Vec<char>>, Position, Position) {
    let mut map = input
        .lines()
        .map(|f| f.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = Position::new();
    let mut end = Position::new();
    map.iter().enumerate().for_each(|(idx, f)| {
        if let Some(pos) = f.iter().position(|c| c == &'S') {
            start = Position::new_value(idx, pos);
        }

        if let Some(pos) = f.iter().position(|c| c == &'E') {
            end = Position::new_value(idx, pos);
        }
    });
    map[start.x][start.y] = 'a';
    map[end.x][end.y] = 'z';
    (map, start, end)
}

fn part_one(input: String) -> String {
    let (map, start, end) = parse_map(input);
    let mut queue: VecDeque<(Position, usize)> = VecDeque::new();
    let mut pass_pos: Vec<Vec<bool>> = Vec::new();

    let x_max = map.len();
    let y_max = map[0].len();

    {
        let mut dummy_bool = Vec::new();
        dummy_bool.resize(y_max, false);
        pass_pos.resize(x_max, dummy_bool);
    }

    queue.push_back((start, 0));
    while !queue.is_empty() {
        let (now_pos, now_move) = queue.pop_front().unwrap();

        if pass_pos[now_pos.x][now_pos.y] {
            continue;
        }

        if now_pos == end {
            return format!("{}", now_move);
        }

        pass_pos[now_pos.x][now_pos.y] = true;

        for (dx, dy) in MOVE_DIFF {
            let new_x = i32::min(i32::max(0, now_pos.x as i32 + dx), x_max as i32 - 1) as usize;
            let new_y = i32::min(i32::max(0, now_pos.y as i32 + dy), y_max as i32 - 1) as usize;
            if !pass_pos[new_x][new_y] {
                let diff = map[new_x][new_y] as i32 - map[now_pos.x][now_pos.y] as i32;
                let new_pos = Position::new_value(new_x, new_y);
                if diff <= 1 {
                    queue.push_back((new_pos, now_move + 1));
                }
            }
        }
    }

    String::new()
}

fn part_two(input: String) -> String {
    let (map, _, end) = parse_map(input);
    let mut queue: VecDeque<(Position, usize)> = VecDeque::new();
    let mut pass_pos: Vec<Vec<bool>> = Vec::new();

    let x_max = map.len();
    let y_max = map[0].len();

    {
        let mut dummy_bool = Vec::new();
        dummy_bool.resize(y_max, false);
        pass_pos.resize(x_max, dummy_bool);
    }

    queue.push_back((end, 0));
    while !queue.is_empty() {
        let (now_pos, now_move) = queue.pop_front().unwrap();

        if map[now_pos.x][now_pos.y] == 'a' {
            return format!("{}", now_move);
        }

        if pass_pos[now_pos.x][now_pos.y] {
            continue;
        }

        pass_pos[now_pos.x][now_pos.y] = true;

        for (dx, dy) in MOVE_DIFF {
            let new_x = i32::min(i32::max(0, now_pos.x as i32 + dx), x_max as i32 - 1) as usize;
            let new_y = i32::min(i32::max(0, now_pos.y as i32 + dy), y_max as i32 - 1) as usize;
            if !pass_pos[new_x][new_y] {
                let diff = map[now_pos.x][now_pos.y] as i32 - map[new_x][new_y] as i32;
                let new_pos = Position::new_value(new_x, new_y);
                if diff <= 1 {
                    queue.push_back((new_pos, now_move + 1));
                }
            }
        }
    }

    String::new()
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day12_test {
    use super::*;
    use std::fs;

    const SAMPLE: &str = "Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        assert_eq!("31", part_one(SAMPLE.to_string()));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day12.txt");
        assert_eq!("412", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        assert_eq!("29", part_two(SAMPLE.to_string()));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day12.txt");
        assert_eq!("402", part_two(input));
    }
}
