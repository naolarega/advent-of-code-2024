fn solution(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> i32 {
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

    similarity_score
}

#[cfg(test)]
mod test {
    use super::{super::read_input, solution};

    #[test]
    fn day_one_part_two_example() {
        let similarity_score = solution(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);

        assert_eq!(similarity_score, 31);
    }

    #[test]
    fn day_one_part_two_with_puzzle_input() {
        let (left_list, right_list) = read_input();
        let similarity_score = solution(left_list, right_list);

        assert_eq!(similarity_score, 20719933);

        println!("Similarity score : {similarity_score}");
    }
}
