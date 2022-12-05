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

fn part1_solve(input: String) {
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

    for i in 0..stock.len() {
        print!("{}", stock[i][stock[i].len() - 1]);
    }
    println!();
}

fn part2_solve(input: String) {
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

    for i in 0..stock.len() {
        print!("{}", stock[i][stock[i].len() - 1]);
    }
    println!();
}

pub fn main(input: String) {
    println!("Part 1");
    part1_solve(input.clone());
    println!("Part 2");
    part2_solve(input);
}
