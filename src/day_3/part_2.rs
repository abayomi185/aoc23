use regex::Regex;

use super::part_1::{look_around, Symbol};

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    let symbols_matrix = input
        .lines()
        .map(|line| {
            let symbols_in_line: Vec<_> = Regex::new(r"[^0-9.]")
                .unwrap()
                .find_iter(line)
                .map(|symbol| Symbol {
                    index: symbol.start() as i32,
                    value: symbol.as_str().chars().next().unwrap(),
                })
                .collect();
            symbols_in_line
        })
        .collect::<Vec<Vec<Symbol>>>();

    let adjacent_numbers = symbols_matrix
        .iter()
        .enumerate()
        .map(|(row, symbols)| {
            symbols
                .iter()
                .map(|symbol| look_around(input, row as i32, symbol.index))
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<Vec<Vec<i32>>>>();

    // println!("{:?}", adjacent_numbers);

    let mut gear_ratios = Vec::new();

    for row in adjacent_numbers {
        for number_array in row {
            if number_array.len() > 1 {
                let filtered_num_array =
                    number_array.iter().filter(|x| *x != &0).collect::<Vec<_>>();
                if filtered_num_array.len() == 2 {
                    let gear_ratio = filtered_num_array[0] * filtered_num_array[1];
                    gear_ratios.push(gear_ratio);
                }
            }
        }
    }

    gear_ratios.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_2() {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#
        .trim_start_matches('\n');

        let result = part_2(input);
        assert_eq!(result, 467835);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("./input.txt");

        let result = part_2(input);
        print_green(&format!("Day 3, part 2 result: {}", result));
    }
}
