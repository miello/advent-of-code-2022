fn part1_solve(input: String) {
    let mut max_val = 0;
    let mut now = 0;
    input.split('\n').for_each(|line| {
        let line_trim = line.trim();
        if line_trim == "" {
            now = 0;
            return;
        }
        let val = line_trim.parse::<i32>().unwrap();
        now += val;
        if now > max_val {
            max_val = now;
        }
    });

    println!("Part 1: {}", max_val);
}

fn part2_solve(input: String) {
    let mut arr: Vec<i32> = Vec::new();
    let mut now = 0;
    input.split('\n').for_each(|line| {
        let line_trim = line.trim();
        if line_trim == "" {
            arr.push(now);
            now = 0;
            return;
        }
        let val = line_trim.parse::<i32>().unwrap();
        now += val;
    });

    arr.push(now);
    arr.sort_by(|a, b| b.cmp(a));

    println!("Part 2: {}", arr[0] + arr[1] + arr[2]);
}

pub fn main(input: String) {
    part1_solve(input.clone());
    part2_solve(input);
}
