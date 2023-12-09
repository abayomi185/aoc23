#[allow(dead_code)]
fn part_2(input: &str) -> i64 {
    let mut sum_vec: Vec<i64> = Vec::new();

    for line in input.lines() {
        let mut matrix: Vec<Vec<i64>> = Vec::new();

        let digits = line
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let initial_first_matrix_row_len = digits.len();

        matrix.push(digits);

        let mut matrix_len = matrix.len();

        while !&matrix[matrix_len - 1].clone().iter().all(|x| x == &0) {
            let difference = &matrix[matrix_len - 1]
                .iter()
                .enumerate()
                .filter_map(|(index, digit)| {
                    matrix[matrix_len - 1]
                        .get(index + 1)
                        .map(|next_digit| next_digit - digit)
                })
                .collect::<Vec<i64>>();

            matrix.push(difference.to_vec());
            matrix_len += 1;
        }

        matrix[matrix_len - 1].insert(0, 0);

        let mut count = 1;

        while matrix[0].len() == initial_first_matrix_row_len {
            let digit_curr = &matrix[(matrix_len - 1) - count].first().unwrap();
            let digit_below = &matrix[(matrix_len - 1) - count + 1].first().unwrap();

            let digit_before = **digit_curr - **digit_below;

            matrix[(matrix_len - 1) - count].insert(0, digit_before);
            count += 1;
        }

        let estimate = matrix[0].first().cloned().unwrap();
        sum_vec.push(estimate);
    }

    sum_vec.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;
    use std::path::Path;

    #[test]
    fn test_part_2() {
        let input = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#
        .trim_start_matches('\n');

        let result = part_2(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("./input.txt");

        let result = part_2(input);

        let file_name = file!();
        let file_path = Path::new(file_name);
        let file_name = file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap();

        let mut dir_name: &str = "";

        if let Some(dir) = file_path.parent() {
            dir_name = dir.to_str().unwrap();
            dir_name = dir_name.split('/').last().unwrap();
        }

        print_green(&format!("{dir_name}, {file_name} result: {}", result));
    }
}
