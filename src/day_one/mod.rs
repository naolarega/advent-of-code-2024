use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod part_one;
mod part_two;

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let input = File::open("./inputs/day_one.txt")
        .expect("day one input file should be in input directory");
    let mut input_buf_read = BufReader::new(input);
    let mut line = String::new();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    while let Ok(byte_read) = input_buf_read.read_line(&mut line) {
        if byte_read == 0 || line.is_empty() {
            break;
        }

        let mut two_halves = line.split("   ");
        let left_half = two_halves
            .next()
            .expect("left half should have had a value")
            .trim()
            .parse::<i32>()
            .expect("left half value should be an integer");
        let right_half = two_halves
            .next()
            .expect("right half should have had a value")
            .trim()
            .parse::<i32>()
            .expect("right half value should be an integer");

        left_list.push(left_half);
        right_list.push(right_half);

        line.clear();
    }

    (left_list, right_list)
}
