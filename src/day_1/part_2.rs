use std::collections::HashMap;

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    let number_map: HashMap<&str, i32> = HashMap::from([
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ]);

    fn get_left_value<'a>(string: &'a str, number_map: &'a HashMap<&str, i32>) -> &'a str {
        for key in number_map.keys() {
            if string.starts_with(key) {
                return key;
            }
        }
        get_left_value(&string[1..], number_map)
    }
    fn get_right_value<'a>(string: &'a str, number_map: &'a HashMap<&str, i32>) -> &'a str {
        for key in number_map.keys() {
            if string.ends_with(key) {
                return key;
            }
        }
        get_right_value(&string[..string.len() - 1], number_map)
    }

    let numbers = input.lines().map(|line| {
        let left = get_left_value(line, &number_map);
        let right = get_right_value(line, &number_map);

        let value = &format!(
            "{}{}",
            number_map.get(left).unwrap(),
            number_map.get(right).unwrap()
        );
        value.parse::<i32>().unwrap()
    });

    numbers.sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_2() {
        let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#
        .trim_start_matches('\n');

        let result = part_2(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("./input.txt");

        let result = part_2(input);
        print_green(&format!("Day 2 result: {result}"));
    }
}
