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

fn part1_solve(input: String) {
    let tree_height = parse_data(input);
    let max_y = tree_height[0].len();
    let max_x = tree_height.len();
    let mut cnt = 0;

    for x in 0..max_x {
        for y in 0..max_y {
            cnt += is_visible(&tree_height, x, y);
        }
    }

    println!("{}", cnt);
}

fn part2_solve(input: String) {
    let tree_height = parse_data(input);
    let max_y = tree_height[0].len();
    let max_x = tree_height.len();
    let mut cnt = 0;

    for x in 0..max_x {
        for y in 0..max_y {
            cnt = i32::max(cnt, calculate_view(&tree_height, x, y));
        }
    }

    println!("{}", cnt);
}

pub fn main(input: String) {
    println!("Part 1");
    part1_solve(input.clone());
    println!("Part 2");
    part2_solve(input);
}
