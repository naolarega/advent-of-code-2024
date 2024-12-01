use super::read_input;

fn solution() {
    let (left_list, right_list) = read_input();

    let similarity_score = left_list
        .iter()
        .map(|left_value| {
            let similarity_score = right_list
                .iter()
                .filter(|right_value| left_value == *right_value)
                .count() as i32;

            left_value * similarity_score
        })
        .sum::<i32>();

    println!("Similarity score : {similarity_score}");
}

#[test]
fn test_day_one_part_two() {
    solution();
}
