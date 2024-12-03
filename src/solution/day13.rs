use std::{cell::RefCell, cmp::Ordering, rc::Rc};

#[derive(Debug, Clone)]
struct Node {
    num: Option<i32>,
    list: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new() -> Node {
        Node {
            num: None,
            list: None,
            next: None,
        }
    }

    fn init_num(&mut self, num: i32) {
        self.num = Some(num);
    }

    fn init_list(&mut self) {
        self.list = Some(Rc::new(RefCell::new(Node::new())));
    }

    fn init_next(&mut self) {
        self.next = Some(Rc::new(RefCell::new(Node::new())));
    }
}

fn parse_node(line: &str) -> Rc<RefCell<Node>> {
    let root_node = Rc::new(RefCell::new(Node::new()));

    let mut node_stack: Vec<Rc<RefCell<Node>>> = Vec::new();
    let mut number: Vec<String> = Vec::new();

    node_stack.push(root_node.clone());

    for ch in line.chars() {
        let tmp = node_stack.last().unwrap().clone();
        let mut node_mut = tmp.borrow_mut();
        if ch == '[' {
            node_mut.init_list();
            if let Some(val) = &node_mut.list {
                node_stack.push(val.clone());
            }
        } else if ch == ']' {
            if number.len() != 0 {
                node_mut.init_num(number.join("").parse::<i32>().unwrap());
                number.clear();
            }
            node_stack.pop();
        } else if ch == ',' {
            if number.len() != 0 {
                node_mut.init_num(number.join("").parse::<i32>().unwrap());
                number.clear();
            }

            node_mut.init_next();
            node_stack.pop();
            if let Some(val) = &node_mut.next {
                node_stack.push(val.clone());
            }
        } else {
            number.push(ch.to_string());
        }
    }

    root_node
}

fn parse_list(input: String) -> (Vec<Rc<RefCell<Node>>>, Vec<Rc<RefCell<Node>>>) {
    let mut left: Vec<Rc<RefCell<Node>>> = Vec::new();
    let mut right: Vec<Rc<RefCell<Node>>> = Vec::new();
    let mut is_left = true;
    for line in input.lines() {
        if line.trim() == "" {
            is_left = true;
            continue;
        }
        let cur_node = parse_node(line.trim());
        if is_left {
            left.push(cur_node);
        } else {
            right.push(cur_node);
        }
        is_left = !is_left;
    }

    (left, right)
}

fn cmp(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) -> Ordering {
    let left_borr = left.borrow();
    let right_borr = right.borrow();

    let left_has_data = !(left_borr.num.is_none() && left_borr.list.is_none());
    let right_has_data = !(right_borr.num.is_none() && right_borr.list.is_none());

    if left_has_data && !right_has_data {
        return Ordering::Greater;
    }

    if !left_has_data && right_has_data {
        return Ordering::Less;
    }

    if left_borr.num.is_some() && right_borr.num.is_some() {
        let cmp_result = left_borr.num.unwrap().cmp(&right_borr.num.unwrap());
        if cmp_result != Ordering::Equal {
            return cmp_result;
        }
    } else if left_has_data && right_has_data {
        let left_node = if left_borr.num.is_some() {
            let mut new_node = Node::new();
            new_node.init_num(left_borr.num.unwrap());
            Rc::new(RefCell::new(new_node))
        } else {
            left_borr.clone().list.unwrap()
        };

        let right_node = if right_borr.num.is_some() {
            let mut new_node = Node::new();
            new_node.init_num(right_borr.num.unwrap());
            Rc::new(RefCell::new(new_node))
        } else {
            right_borr.clone().list.unwrap()
        };

        let cmp_result = cmp(left_node, right_node);
        if cmp_result != Ordering::Equal {
            return cmp_result;
        }
    }

    if left_borr.next.is_some() && right_borr.next.is_some() {
        return cmp(
            left_borr.clone().next.unwrap(),
            right_borr.clone().next.unwrap(),
        );
    }

    if left_borr.next.is_some() {
        return Ordering::Greater;
    }

    if right_borr.next.is_some() {
        return Ordering::Less;
    }

    Ordering::Equal
}

fn part_one(input: String) -> String {
    let (left, right) = parse_list(input);

    let len = left.len();
    let mut sum = 0;
    for idx in 0..len {
        let result = cmp(left[idx].clone(), right[idx].clone());
        if result == Ordering::Less {
            sum += idx + 1;
        }
    }
    format!("{}", sum)
}

fn part_two(input: String) -> String {
    let (left, right) = parse_list(input);
    let mut combine_node = Vec::new();

    combine_node.extend(left);
    combine_node.extend(right);
    combine_node.sort_by(|a, b| cmp(a.clone(), b.clone()));

    let first_key_node = parse_node("[[2]]");
    let second_key_node = parse_node("[[6]]");

    let mut answer = 1;

    let len = combine_node.len();

    for idx in 0..len {
        if cmp(combine_node[idx].clone(), first_key_node.clone()) == Ordering::Greater {
            answer *= idx + 1;
            combine_node.insert(idx, first_key_node.clone());
            break;
        }
    }

    for idx in 0..len {
        if cmp(combine_node[idx].clone(), second_key_node.clone()) == Ordering::Greater {
            answer *= idx + 1;
            combine_node.insert(idx, first_key_node.clone());
            break;
        }
    }

    format!("{}", answer)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day13_test {
    use super::*;
    use std::fs;

    fn read_testcase(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    const SAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part_one_sample_test() {
        assert_eq!("13", part_one(SAMPLE.to_string()));
    }

    #[test]
    fn part_one_real_test() {
        let input = read_testcase("testcase/day13.txt");
        assert_eq!("5292", part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        assert_eq!("140", part_two(SAMPLE.to_string()));
    }

    #[test]
    fn part_two_real_test() {
        let input = read_testcase("testcase/day13.txt");
        assert_eq!("23868", part_two(input));
    }
}
