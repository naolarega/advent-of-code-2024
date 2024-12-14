use super::{check_safety, Safety};

fn solution(reports: Vec<Vec<i32>>) -> i32 {
    let safe_reports = reports
        .iter()
        .map(|levels| check_safety(levels))
        .filter(|safety| *safety == Safety::Safe)
        .count() as i32;

    safe_reports
}

#[cfg(test)]
mod tests {
    use super::{super::read_input, solution};

    #[test]
    fn day_two_part_one_example() {
        let safe_reports = solution(vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]);

        assert_eq!(safe_reports, 2);
    }

    #[test]
    fn day_two_part_one_with_puzzle_input() {
        let reports = read_input();
        let safe_reports = solution(reports);

        assert_eq!(safe_reports, 479);

        println!("Safe reports : {safe_reports}");
    }
}
