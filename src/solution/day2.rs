fn convert_opponent_to_mine(opponent: &str) -> &str {
    match opponent {
        "A" => "X",
        "B" => "Y",
        "C" => "Z",
        _ => "",
    }
}

fn get_score(mine: &str, opponent: &str) -> i32 {
    let mut current_score = 0;
    match mine {
        "X" => {
            current_score += 1;

            if opponent == "C" {
                current_score += 6;
            }

            if opponent == "A" {
                current_score += 3;
            }
        }
        "Y" => {
            current_score += 2;

            if opponent == "A" {
                current_score += 6;
            }

            if opponent == "B" {
                current_score += 3;
            }
        }
        "Z" => {
            current_score += 3;

            if opponent == "B" {
                current_score += 6;
            }

            if opponent == "C" {
                current_score += 3;
            }
        }
        _ => {}
    };
    return current_score;
}

fn part1_solve(input: String) {
    let mut current_score = 0;
    for line in input.split("\r\n") {
        let mut split = line.split(" ");
        let opponent = split.next().unwrap();
        let mine = split.next().unwrap();
        current_score += get_score(mine, opponent);
    }
    println!("{}", current_score);
}

fn part2_solve(input: String) {
    let mut current_score = 0;
    for line in input.split("\r\n") {
        let mut split = line.split(" ");
        let opponent = split.next().unwrap();
        let result_status = split.next().unwrap();
        let mut mine = "";
        match result_status {
            "X" => match opponent {
                "A" => mine = "C",
                "B" => mine = "A",
                "C" => mine = "B",
                _ => {}
            },
            "Y" => {
                mine = opponent;
            }
            "Z" => match opponent {
                "A" => mine = "B",
                "B" => mine = "C",
                "C" => mine = "A",
                _ => {}
            },
            _ => {}
        };

        current_score += get_score(convert_opponent_to_mine(mine), opponent);
    }

    println!("{}", current_score);
}

pub fn main(input: String) {
    println!("Part 1");
    part1_solve(input.clone());
    println!("Part 2");
    part2_solve(input);
}
