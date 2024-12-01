use super::read_input;

fn solution() {
    let (mut left_list, mut right_list) = read_input();

    left_list.sort();
    right_list.sort();

    let distance = left_list
        .iter()
        .zip(&right_list)
        .map(|(left_half, right_half)| (left_half - right_half).abs())
        .sum::<i32>();

    println!("Distance : {distance}");
}

#[test]
fn test_day_one_part_one() {
    solution();
}
