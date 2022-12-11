struct Monkey {
    cnt_ops: i32,
    raw_ops_fn: Vec<String>,
    result: [usize; 2],
    div: i64,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            cnt_ops: 0,
            raw_ops_fn: Vec::new(),
            result: [0, 0],
            div: 1,
        }
    }

    fn init_ops_fn(&mut self, raw_ops: &str) {
        self.raw_ops_fn = raw_ops
            .split(" ")
            .skip(2)
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
    }

    fn init_test_fn(&mut self, raw_test: &str) {
        self.div = raw_test.split(" ").last().unwrap().parse::<i64>().unwrap();
    }

    fn init_result(&mut self, condition: &str, result: &str) {
        let next_monkey = result.split(" ").last().unwrap().parse::<usize>().unwrap();
        if condition.contains("true") {
            self.result[1] = next_monkey;
        }
        if condition.contains("false") {
            self.result[0] = next_monkey;
        }
    }

    fn get_next_monkey(&self, num: i64) -> usize {
        match num % self.div == 0 {
            true => self.result[1],
            false => self.result[0],
        }
    }

    fn calculate_next_value(&self, num: i64) -> i64 {
        let a = match self.raw_ops_fn[0].as_str() {
            "old" => num,
            _ => self.raw_ops_fn[0].parse::<i64>().unwrap(),
        };

        let b = match self.raw_ops_fn[2].as_str() {
            "old" => num,
            _ => self.raw_ops_fn[2].parse::<i64>().unwrap(),
        };

        let cal = match self.raw_ops_fn[1].as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => 0,
        };

        cal
    }
}

fn parse_data(input: String) -> (Vec<Monkey>, Vec<(i64, usize)>) {
    let mut hs = Vec::new();
    let mut items = Vec::new();
    input
        .lines()
        .map(|f| f.trim().split(": ").collect::<Vec<&str>>())
        .for_each(|f| {
            let ops = f[0];
            if ops.starts_with("Monkey") {
                hs.push(Monkey::new());
                return;
            }

            let now_len = hs.len();
            let test = &mut hs[now_len - 1];

            if ops == "Starting items" {
                items.extend(
                    f[1].split(", ")
                        .map(|f| (f.parse::<i64>().unwrap(), now_len - 1)),
                );
                return;
            }

            if ops == "Operation" {
                test.init_ops_fn(f[1]);
                return;
            }

            if ops == "Test" {
                test.init_test_fn(f[1]);
                return;
            }

            if ops.starts_with("If") {
                test.init_result(ops, f[1]);
                return;
            }
        });
    (hs, items)
}

fn part_one(input: String) -> String {
    let (mut monkey_worker, mut items) = parse_data(input);
    let monkey_len = monkey_worker.len();
    let item_len = items.len();
    for _ in 0..20 {
        for idx in 0..monkey_len {
            let mut found: usize = 0;
            for idx_item in 0..item_len {
                let (val, worker_idx) = items[idx_item - found];
                if worker_idx == idx {
                    let next_val = monkey_worker[idx].calculate_next_value(val) / 3;
                    let next_monkey = monkey_worker[idx].get_next_monkey(next_val);
                    items.remove(idx_item - found);
                    items.push((next_val, next_monkey));
                    monkey_worker[idx].cnt_ops += 1;
                    found += 1;
                }
            }
        }
    }

    monkey_worker.sort_by_key(|f| -f.cnt_ops);
    format!("{}", monkey_worker[0].cnt_ops * monkey_worker[1].cnt_ops)
}

fn part_two(input: String) -> String {
    let (mut monkey_worker, items) = parse_data(input);

    // We will use different strategy to process part 2
    let mut items_modified: Vec<(Vec<i64>, usize)> = items
        .iter()
        .map(|f| {
            let val = f.0;
            (
                monkey_worker
                    .iter()
                    .map(|x| val % x.div)
                    .collect::<Vec<i64>>(),
                f.1,
            )
        })
        .collect();

    let monkey_len = monkey_worker.len();
    let item_len = items.len();
    for _ in 0..10000 {
        for idx in 0..monkey_len {
            let mut found: usize = 0;
            for idx_item in 0..item_len {
                let (val, worker_idx) = &items_modified[idx_item - found];
                if *worker_idx == idx {
                    let next_val = val
                        .iter()
                        .enumerate()
                        .map(|(i, f)| {
                            monkey_worker[idx].calculate_next_value(*f) % monkey_worker[i].div
                        })
                        .collect::<Vec<i64>>();

                    let next_monkey = monkey_worker[idx].get_next_monkey(next_val[idx]);

                    items_modified.remove(idx_item - found);
                    items_modified.push((next_val, next_monkey));
                    monkey_worker[idx].cnt_ops += 1;
                    found += 1;
                }
            }
        }
    }

    monkey_worker.sort_by_key(|f| -f.cnt_ops);
    format!(
        "{}",
        monkey_worker[0].cnt_ops as i128 * monkey_worker[1].cnt_ops as i128
    )
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day11_test {
    use super::*;
    use std::fs;

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    #[test]
    fn part_one_sample_test() {
        let input = read_testcase("testcase/day11_sample.txt");
        assert_eq!("10605", part_one(input.to_string()));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day11.txt");
        assert_eq!("55930", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let input = read_testcase("testcase/day11_sample.txt");
        assert_eq!("2713310158", part_two(input.to_string()));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day11.txt");
        assert_eq!("14636993466", part_two(input));
    }
}
