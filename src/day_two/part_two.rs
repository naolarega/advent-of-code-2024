use super::{check_safety, Safety};

fn solution(mut reports: Vec<Vec<i32>>) -> i32 {
    let safe_reports = reports
        .iter_mut()
        .map(|levels| {
            let mut levels_clone = levels.clone();

            for level_index in 0..levels.len() {
                levels_clone.remove(level_index);

                if check_safety(&levels_clone).is_safe() {
                    return Safety::Safe;
                }

                levels_clone = levels.clone();
            }

            Safety::Unsafe
        })
        .filter(|safety| *safety == Safety::Safe)
        .count() as i32;

    safe_reports
}

#[cfg(test)]
mod tests {
    use super::{super::read_input, solution};

    #[test]
    fn day_two_part_two_example() {
        let safe_reports = solution(vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]);

        assert_eq!(safe_reports, 4);
    }

    #[test]
    fn day_two_part_two_with_puzzle_input_test() {
        let reports = read_input();
        let safe_reports = solution(reports);

        // assert_eq!(safe_reports, 506);

        println!("Safe reports : {safe_reports}");
    }
}
