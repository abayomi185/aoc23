use regex::Regex;
use std::collections::HashSet;

pub fn get_points(input: &str) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = Vec::new();

    for line in input.lines() {
        let colon_split = line.split(':').collect::<Vec<&str>>();

        let card_no = Regex::new(r"\d+")
            .unwrap()
            .find(colon_split[0])
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        let vertical_bar_split = colon_split[1].split('|').collect::<Vec<&str>>();

        let winning_numbers = Regex::new(r"\d+")
            .unwrap()
            .find_iter(vertical_bar_split[0])
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect::<HashSet<i32>>();

        let numbers_to_check = Regex::new(r"\d+")
            .unwrap()
            .find_iter(vertical_bar_split[1])
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect::<HashSet<i32>>();

        let point = (winning_numbers.intersection(&numbers_to_check).count() as i32) - 1;

        if point > -1 {
            points.push((card_no, i32::pow(2, point as u32)));
        } else {
            points.push((card_no, 0));
        }
    }

    points
}

#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    get_points(input).iter().map(|(_, point)| point).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_1() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#
        .trim_start_matches('\n');

        let result = part_1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("./input.txt");

        let result = part_1(input);
        print_green(&format!("Day 4, part 1 result: {}", result));
    }
}
