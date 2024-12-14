use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod part_one;
mod part_two;

#[derive(PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

impl Safety {
    fn is_safe(self) -> bool {
        self == Safety::Safe
    }
}

fn read_input() -> Vec<Vec<i32>> {
    let input = File::open("./inputs/day_two.txt")
        .expect("day two input file should be in the inputs directory");
    let mut input_buf_reader = BufReader::new(input);
    let mut reports = Vec::new();
    let mut line = String::new();

    while let Ok(bytes_read) = input_buf_reader.read_line(&mut line) {
        if bytes_read == 0 || line.is_empty() {
            break;
        }

        let levels = line
            .split_ascii_whitespace()
            .map(|level| {
                level
                    .trim()
                    .parse::<i32>()
                    .expect("level should be an integer")
            })
            .collect::<Vec<i32>>();
        reports.push(levels);

        line.clear();
    }

    reports
}

fn check_safety(levels: &Vec<i32>) -> Safety {
    if !levels.is_sorted_by(|a, b| a >= b) && !levels.is_sorted_by(|a, b| a <= b) {
        return Safety::Unsafe;
    }

    let safe = levels
        .windows(2)
        .map(|levels| (levels[0] - levels[1]).abs())
        .all(|difference| difference >= 1 && difference <= 3);

    if safe {
        Safety::Safe
    } else {
        Safety::Unsafe
    }
}
