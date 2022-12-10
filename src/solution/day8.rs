fn parse_data(inp: String) -> Vec<Vec<u32>> {
    inp.lines()
        .map(|f| {
            f.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn is_visible(tree_height: &Vec<Vec<u32>>, x: usize, y: usize) -> i32 {
    let max_y = tree_height[0].len();
    let max_x = tree_height.len();
    let mut visible: i32 = 4;
    for new_x in 0..x {
        if tree_height[new_x][y] >= tree_height[x][y] {
            visible -= 1;
            break;
        }
    }

    for new_x in (x + 1)..max_x {
        if tree_height[new_x][y] >= tree_height[x][y] {
            visible -= 1;
            break;
        }
    }

    for new_y in 0..y {
        if tree_height[x][new_y] >= tree_height[x][y] {
            visible -= 1;
            break;
        }
    }

    for new_y in (y + 1)..max_y {
        if tree_height[x][new_y] >= tree_height[x][y] {
            visible -= 1;
            break;
        }
    }

    i32::min(visible, 1)
}

fn calculate_view(tree_height: &Vec<Vec<u32>>, x: usize, y: usize) -> i32 {
    let max_y = tree_height[0].len();
    let max_x = tree_height.len();
    let mut visible_cnt = 0;
    let mut now = 1;

    if y == max_y - 1 || y == 0 || x == 0 || x == max_x - 1 {
        return 0;
    }

    for new_x in (0..x).rev() {
        visible_cnt += 1;
        if tree_height[new_x][y] >= tree_height[x][y] {
            break;
        }
    }
    now *= visible_cnt;

    visible_cnt = 0;
    for new_x in (x + 1)..max_x {
        visible_cnt += 1;
        if tree_height[new_x][y] >= tree_height[x][y] {
            break;
        }
    }
    now *= visible_cnt;

    visible_cnt = 0;
    for new_y in (0..y).rev() {
        visible_cnt += 1;
        if tree_height[x][new_y] >= tree_height[x][y] {
            break;
        }
    }
    now *= visible_cnt;

    visible_cnt = 0;
    for new_y in (y + 1)..max_y {
        visible_cnt += 1;
        if tree_height[x][new_y] >= tree_height[x][y] {
            break;
        }
    }
    now *= visible_cnt;

    now
}

fn part_one(input: String) -> String {
    let tree_height = parse_data(input);
    let max_y = tree_height[0].len();
    let max_x = tree_height.len();
    let mut cnt = 0;

    for x in 0..max_x {
        for y in 0..max_y {
            cnt += is_visible(&tree_height, x, y);
        }
    }

    format!("{}", cnt)
}

fn part_two(input: String) -> String {
    let tree_height = parse_data(input);
    let max_y = tree_height[0].len();
    let max_x = tree_height.len();
    let mut cnt = 0;

    for x in 0..max_x {
        for y in 0..max_y {
            cnt = i32::max(cnt, calculate_view(&tree_height, x, y));
        }
    }

    format!("{}", cnt)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day8_test {
    use super::*;
    use std::fs;

    const SAMPLE: &str = "30373
25512
65332
33549
35390";

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        assert_eq!("21", part_one(SAMPLE.to_string()));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day8.txt");
        assert_eq!("1818", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        assert_eq!("8", part_two(SAMPLE.to_string()));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day8.txt");
        assert_eq!("368368", part_two(input));
    }
}
