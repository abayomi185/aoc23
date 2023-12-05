use regex::Regex;

#[derive(Debug)]
pub struct Symbol {
    pub index: i32,
    pub value: char,
}

fn get_number_around_index(line: &str, index: usize) -> Vec<i32> {
    let numbers = Regex::new(r"\d+")
        .unwrap()
        .find_iter(line)
        .map(|number| {
            let index_range = (index - 1)..=(index + 1);

            match number.start() <= *index_range.end() && number.end() > *index_range.start() {
                true => number.as_str().parse::<i32>().unwrap(),
                false => 0,
            }
        })
        .collect();
    numbers
}

fn get_number_at_index(line: &str, index: usize) -> Vec<i32> {
    let numbers = Regex::new(r"\d+")
        .unwrap()
        .find_iter(line)
        .map(
            |number| match number.start() <= index && number.end() >= index {
                true => number.as_str().parse::<i32>().unwrap(),
                false => 0,
            },
        )
        .collect();
    numbers
}

pub fn look_around(input: &str, row: i32, index: i32) -> Vec<i32> {
    let mut finds = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    let max_row = lines.len() as i32;

    if row > 0 {
        // Check up
        let slice = &lines[(row - 1) as usize]
            [(index - 1).try_into().unwrap()..=(index + 1).try_into().unwrap()];
        if slice.chars().any(|c| c != '.') {
            let numbers = get_number_around_index(lines[(row - 1) as usize], index as usize);
            finds.extend(numbers)
        }
    }
    if row < max_row {
        // Check down
        let slice = &lines[(row + 1) as usize]
            [(index - 1).try_into().unwrap()..=(index + 1).try_into().unwrap()];
        if slice.chars().any(|c| c != '.') {
            let numbers = get_number_around_index(lines[(row + 1) as usize], index as usize);
            finds.extend(numbers)
        }
    }

    let left = lines[row as usize]
        .chars()
        .nth((index - 1).try_into().unwrap())
        .unwrap();
    if left != '.' {
        let numbers = get_number_at_index(lines[row as usize], (index - 1) as usize);
        finds.extend(numbers);
    }

    let right = lines[row as usize]
        .chars()
        .nth((index + 1).try_into().unwrap())
        .unwrap();
    if right != '.' {
        let numbers = get_number_at_index(lines[row as usize], (index + 1) as usize);
        finds.extend(numbers);
    }

    finds
}

#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
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
                .flat_map(|symbol| look_around(input, row as i32, symbol.index))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    // println!("{:?}", adjacent_numbers);

    adjacent_numbers.iter().flatten().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_1() {
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

        let result = part_1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("./input.txt");

        let result = part_1(input);
        print_green(&format!("Day 3, part 1 result: {}", result));
    }
}
