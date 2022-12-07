use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct File {
    name: String,
    size: i32,
}

struct Folder {
    name: String,
    size: i32,
    sub_folder: HashMap<String, Rc<RefCell<Folder>>>,
    files: Vec<File>,
}

fn generate_folder_map(input: String) -> Rc<RefCell<Folder>> {
    let mut folder_stack: Vec<Rc<RefCell<Folder>>> = Vec::new();
    let root_folder = Rc::new(RefCell::new(Folder {
        name: String::from("/"),
        size: 0,
        sub_folder: HashMap::new(),
        files: Vec::new(),
    }));

    let mut folder = root_folder.clone();

    folder_stack.push(Rc::clone(&folder));

    let lines = input.lines();
    for line in lines {
        let split_line = line.split(' ').collect::<Vec<&str>>();
        if line.starts_with('$') {
            match split_line[1] {
                "cd" => {
                    if split_line[2] == ".." {
                        folder_stack.pop();
                        folder = folder_stack[folder_stack.len() - 1].clone();
                    } else if split_line[2] != "/" {
                        let tmp = folder.borrow().sub_folder[split_line[2]].clone();
                        folder_stack.push(tmp.clone());
                        folder = tmp;
                    }
                }
                _ => {}
            }
            continue;
        }

        let mut folder_mut = folder.borrow_mut();
        if line.starts_with("dir") {
            folder_mut.sub_folder.insert(
                String::from(split_line[1]),
                Rc::new(RefCell::new(Folder {
                    name: String::from(split_line[1]),
                    size: 0,
                    sub_folder: HashMap::new(),
                    files: Vec::new(),
                })),
            );

            continue;
        }

        let file_sz = split_line[0].parse::<i32>().unwrap();
        let file_name = split_line[1];

        folder_mut.files.push(File {
            name: String::from(file_name),
            size: file_sz,
        })
    }

    root_folder
}

fn fill_folder_size(folder: &Rc<RefCell<Folder>>) -> i32 {
    let mut folder_bor = folder.borrow_mut();
    let mut final_sz = 0;
    for file in &folder_bor.files {
        final_sz += file.size;
    }

    for (_, sub_folder) in &folder_bor.sub_folder {
        final_sz += fill_folder_size(&sub_folder);
    }

    folder_bor.size = final_sz;
    final_sz
}

fn find_sum_less_than(folder: &Rc<RefCell<Folder>>, limit: i32) -> i32 {
    let folder_bor = folder.borrow();
    let mut cnt = 0;
    for (_, sub_folder) in &folder_bor.sub_folder {
        let sub_borrow = sub_folder.borrow();
        if sub_borrow.size <= limit {
            cnt += sub_borrow.size
        }
        cnt += find_sum_less_than(&sub_folder, limit);
    }

    cnt
}

fn folder_traversal(folder: &Rc<RefCell<Folder>>) {
    let folder_bor = folder.borrow();
    println!("Directory {} {}", folder_bor.name, folder_bor.size);
    for file in &folder_bor.files {
        println!("File: {} {}", file.name, file.size);
    }

    for (_, sub_folder) in &folder_bor.sub_folder {
        folder_traversal(&sub_folder);
    }
}

fn find_min_exceed_limit(folder: &Rc<RefCell<Folder>>, limit: i32) -> i32 {
    let folder_bor = folder.borrow();
    let mut cnt = i32::MAX;
    if folder_bor.size >= limit {
        cnt = folder_bor.size;
    }
    for (_, sub_folder) in &folder_bor.sub_folder {
        let sub_borrow = sub_folder.borrow();
        if sub_borrow.size >= limit {
            cnt = i32::min(cnt, sub_borrow.size);
        }

        cnt = i32::min(cnt, find_min_exceed_limit(&sub_folder, limit));
    }

    cnt
}

fn part1_solve(input: String) {
    let folder_structure = generate_folder_map(input);
    fill_folder_size(&folder_structure);
    println!("{}", find_sum_less_than(&folder_structure, 100000));
}

fn part2_solve(input: String) {
    let folder_structure = generate_folder_map(input);
    fill_folder_size(&folder_structure);

    let deleted_size = i32::max(folder_structure.borrow().size - 40000000, 0);
    println!("{}", find_min_exceed_limit(&folder_structure, deleted_size));
}

pub fn main(input: String) {
    println!("Part 1");
    part1_solve(input.clone());
    println!("Part 2");
    part2_solve(input);
}
