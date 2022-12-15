use std::collections::HashSet;

struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Position {
        Position { x, y }
    }

    fn distance(&self, other: &Position) -> i64 {
        i64::abs(self.x - other.x) + i64::abs(self.y - other.y)
    }
}

struct Sensor {
    position: Position,
    closest_beacon: Position,
}

impl Sensor {
    fn new(position: Position, closest_beacon: Position) -> Sensor {
        Sensor {
            position,
            closest_beacon,
        }
    }
}

fn parse_subdata(text: &str) -> Position {
    let split_data = text.split(" ").collect::<Vec<&str>>();
    let x = split_data[split_data.len() - 2]
        .strip_prefix("x=")
        .unwrap()
        .strip_suffix(",")
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let y = split_data[split_data.len() - 1]
        .strip_prefix("y=")
        .unwrap()
        .parse::<i64>()
        .unwrap();

    Position::new(x, y)
}

fn parse_sensor(input: String) -> Vec<Sensor> {
    let mut sensor_beacon: Vec<Sensor> = Vec::new();

    for line in input.lines() {
        let mut split_iter = line.split(": ");
        let sensor = split_iter.next().unwrap();
        let beacon = split_iter.next().unwrap();

        sensor_beacon.push(Sensor::new(parse_subdata(sensor), parse_subdata(beacon)));
    }

    sensor_beacon
}

fn get_unavailable_range(sensor_data: &Vec<Sensor>, limit_y: i64) -> Vec<(i64, i64)> {
    let mut range_not_working: Vec<(i64, i64)> = Vec::new();
    let mut compress: Vec<(i64, i64)> = Vec::new();

    for sensor in sensor_data {
        let dist = Position::distance(&sensor.position, &sensor.closest_beacon);
        if i64::abs(sensor.position.y - limit_y) > dist {
            continue;
        }
        let remain_dist = dist - i64::abs(sensor.position.y - limit_y);
        range_not_working.push((
            sensor.position.x - remain_dist,
            sensor.position.x + remain_dist,
        ));
    }

    let len = range_not_working.len();
    range_not_working.sort();

    if range_not_working.len() == 0 {
        return compress;
    }

    let mut cur = range_not_working[0];

    for idx in 1..len {
        if cur.1 + 1 < range_not_working[idx].0 {
            compress.push(cur);
            cur = range_not_working[idx];
        } else {
            cur = (cur.0, i64::max(range_not_working[idx].1, cur.1));
        }
    }
    compress.push(cur);

    compress
}

fn part_one_solver(input: String, limit_y: i64) -> String {
    let sensor_data = parse_sensor(input);
    let compress_data = get_unavailable_range(&sensor_data, limit_y);

    let mut beacon_pos: HashSet<i64> = HashSet::new();

    for sensor in &sensor_data {
        if sensor.closest_beacon.y == limit_y {
            beacon_pos.insert(sensor.closest_beacon.x);
        }
    }

    let mut total = -(beacon_pos.len() as i64);

    for (a, b) in compress_data {
        total += b - a + 1;
    }

    format!("{}", total)
}

fn part_two_solver(input: String, limit: i64) -> String {
    let sensor_data = parse_sensor(input);
    let right = limit as usize + 1;
    let mut x: i64 = 0;
    let mut y = 0;
    for limit_y in 0..right {
        let unavailable = get_unavailable_range(&sensor_data, limit_y as i64);
        if unavailable.len() == 1 {
            if unavailable[0].1 < limit {
                x = limit;
                y = limit_y as i64;
            }
            if unavailable[0].0 > 0 {
                x = 0;
                y = limit_y as i64;
            }
        }
        if unavailable.len() == 2 {
            x = unavailable[0].1 + 1;
            y = limit_y as i64;
            break;
        }
    }

    format!("{}", 4000000 * x + y)
}

fn part_one(input: String) -> String {
    part_one_solver(input, 2000000)
}

fn part_two(input: String) -> String {
    part_two_solver(input, 4000000)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day15_test {
    use super::*;
    use std::fs;

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        let input = read_testcase("testcase/day15_sample.txt");
        assert_eq!("26", part_one_solver(input, 10));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day15.txt");
        assert_eq!("5142231", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let input = read_testcase("testcase/day15_sample.txt");
        assert_eq!("56000011", part_two_solver(input, 20));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day15.txt");
        assert_eq!("10884459367718", part_two(input));
    }
}
