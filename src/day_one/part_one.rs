fn solution(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> i32 {
    left_list.sort();
    right_list.sort();

    let distance = left_list
        .iter()
        .zip(&right_list)
        .map(|(left_half, right_half)| (left_half - right_half).abs())
        .sum::<i32>();

    distance
}

#[cfg(test)]
mod tests {
    use super::{super::read_input, solution};

    #[test]
    fn test_day_one_part_one_example() {
        let distance = solution(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);

        assert_eq!(distance, 11);
    }

    #[test]
    fn day_one_part_one_with_puzzle_input() {
        let (left_list, right_list) = read_input();
        let distance = solution(left_list, right_list);

        assert_eq!(distance, 2164381);

        println!("Distance : {distance}");
    }
}
