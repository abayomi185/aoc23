use std::iter::zip;

use super::part_1::{get_winning_ways, Race};

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    let sections = input.split_once('\n').unwrap();

    println!(
        "times: {:?}",
        sections.0.split_whitespace().skip(1).collect::<Vec<_>>()
    );

    println!(
        "distances: {:?}",
        sections.1.split_whitespace().skip(1).collect::<Vec<_>>()
    );

    let times = vec![sections
        .0
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .as_str()
        .parse::<i64>()
        .unwrap()];

    let distances = vec![sections
        .1
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .as_str()
        .parse::<i64>()
        .unwrap()];

    let races = zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<Race>>();

    races
        .iter()
        .map(|race| {
            // Stop the cap - count
            get_winning_ways(race).len() as i32
        })
        .collect::<Vec<_>>()
        .iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_2() {
        let input = r#"
Time:      7  15   30
Distance:  9  40  200
"#
        .trim_start_matches('\n');

        let result = part_2(input);
        assert_eq!(result, 71503);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("./input.txt");

        let result = part_2(input);
        print_green(&format!("Day 6, part 2 result: {}", result));
    }
}
