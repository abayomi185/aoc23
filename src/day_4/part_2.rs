use super::part_1::get_points;

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    let mut points = get_points(input);
    let mut row = 0;

    while row < points.len() {
        if points[row].1 > 0 {
            let matching_numbers = points[row].1.ilog2() as usize + 1;

            let start_index = points[row].0 as usize;
            let end_index = start_index + matching_numbers;

            let extension_range = start_index..end_index;

            points.extend_from_within(extension_range);
        }

        row += 1;
    }

    points.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_2() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#
        .trim_start_matches('\n');

        let result = part_2(input);
        assert_eq!(result, 30);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("./input.txt");

        let result = part_2(input);
        print_green(&format!("Day 4, part 2 result: {}", result));
    }
}
