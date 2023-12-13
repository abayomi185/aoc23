use std::ops::Range;

use crate::util::Point;

#[derive(Clone, Copy, Debug)]
pub struct Galaxy {
    pub _id: i64,
    pub position: Point,
}

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut transposed_matrix = vec![vec![' '; rows]; cols];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed_matrix[j][i] = val;
        }
    }
    transposed_matrix
}

#[allow(dead_code)]
fn get_columns_without_galaxy_in_range(input: &str, range: Range<i64>) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut columns_without_galaxy = 0;

    for index in range {
        let mut has_galaxy = false;
        for line in &lines {
            if let Some(char) = line.chars().nth(index as usize) {
                if char == '#' {
                    has_galaxy = true;
                    break;
                }
            }
        }
        if !has_galaxy {
            columns_without_galaxy += 1;
        }
    }

    columns_without_galaxy
}
pub fn expand_columns(input: &mut Vec<Vec<char>>) -> Vec<i64> {
    let mut transposed_input = transpose(input.to_owned());

    let expanded_columns = expand_rows(&mut transposed_input);

    *input = transpose(transposed_input);

    expanded_columns
}

#[allow(dead_code)]
fn get_rows_without_galaxy_in_range(input: &str, range: Range<i64>) -> i64 {
    let mut rows_without_galaxy = 0;
    for (index, line) in input.lines().enumerate() {
        if !range.contains(&(index as i64)) {
            continue;
        }
        let has_galaxy = line.chars().any(|char| char == '#');
        if !has_galaxy {
            rows_without_galaxy += 1;
        }
    }
    rows_without_galaxy
}
pub fn expand_rows(input: &mut Vec<Vec<char>>) -> Vec<i64> {
    let mut rows_to_expand: Vec<i64> = Vec::new();

    for (index, line) in input.iter_mut().enumerate() {
        if line.iter().all(|char| *char == '.') {
            rows_to_expand.push(index as i64);
        }
    }
    for (index, row) in rows_to_expand.iter().enumerate() {
        input.insert(*row as usize + index, vec!['.'; input[0].len()]);
    }

    rows_to_expand
}

#[allow(dead_code)]
fn part_1(input: &str) -> i64 {
    let mut galaxies: Vec<Galaxy> = Vec::new();

    let mut count = 0;

    // Expand the universe when all values are .
    let mut input_matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let expanded_rows = expand_rows(&mut input_matrix);
    let expanded_columns = expand_columns(&mut input_matrix);

    println!("expanded_rows: {:?}", expanded_rows);
    println!("expanded_columns: {:?}", expanded_columns);

    let new_strings = input_matrix
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    let new_input = new_strings.as_str();

    for (row, line) in new_input.lines().enumerate() {
        for (index, char) in line.chars().enumerate() {
            if char == '#' {
                count += 1;
                galaxies.push(Galaxy {
                    _id: count,
                    position: Point::new(index as i64, row as i64),
                });
            }
        }
    }

    let mut galaxy_pairs: Vec<(Galaxy, Galaxy)> = Vec::new();

    for (i, galaxy_1) in galaxies.iter().enumerate() {
        for galaxy_2 in galaxies.iter().skip(i + 1) {
            galaxy_pairs.push((*galaxy_1, *galaxy_2));
        }
    }

    let mut shortest_distances: Vec<i64> = Vec::new();

    for (galaxy_1, galaxy_2) in galaxy_pairs {
        let delta = galaxy_1.position - galaxy_2.position;

        let shortest_distance = delta.x.abs() + delta.y.abs();
        shortest_distances.push(shortest_distance);
    }

    shortest_distances.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;
    use std::path::Path;

    #[test]
    fn test_part_1() {
        let input = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#
        .trim_start_matches('\n');

        let result = part_1(input);
        assert_eq!(result, 374);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("./input.txt");

        let result = part_1(input);

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
