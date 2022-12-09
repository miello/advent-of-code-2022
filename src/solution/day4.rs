struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn is_fully_inside(&self, other: &Point) -> bool {
        self.x <= other.x && self.y >= other.y
    }

    fn is_overlap(&self, other: &Point) -> bool {
        !(self.x > other.y || self.y < other.x)
    }
}

fn split_data(input: String) -> Point {
    let mut split = input.split("-");
    let x = split.next().unwrap().parse::<i32>().unwrap();
    let y = split.next().unwrap().parse::<i32>().unwrap();
    return Point::new(x, y);
}

fn part1_solve(input: String) {
    let mut fully = 0;
    for line in input.lines() {
        let mut spliting = line.split(',');
        let first_point = split_data(spliting.next().unwrap().to_string());
        let second_point = split_data(spliting.next().unwrap().to_string());

        if second_point.is_fully_inside(&first_point) {
            fully += 1;
            continue;
        }

        if first_point.is_fully_inside(&second_point) {
            fully += 1;
            continue;
        }
    }

    println!("{}", fully);
}

fn part2_solve(input: String) {
    let mut fully = 0;
    for line in input.lines() {
        let mut spliting = line.split(',');
        let first_point = split_data(spliting.next().unwrap().to_string());
        let second_point = split_data(spliting.next().unwrap().to_string());

        if first_point.is_overlap(&second_point) {
            fully += 1;
            continue;
        }
    }

    println!("{}", fully);
}

pub fn main(input: String) {
    println!("Part 1");
    part1_solve(input.clone());
    println!("Part 2");
    part2_solve(input);
}
