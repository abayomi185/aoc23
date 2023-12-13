use crate::util::Point;
use std::cmp::{max, min};
use std::ops::Range;

use super::part_1::{expand_columns, expand_rows, Galaxy};

#[allow(dead_code)]
fn part_2(input: &str, multiplier: i64) -> i64 {
    let mut galaxies: Vec<Galaxy> = Vec::new();

    let mut count = 0;

    // Expand the universe when all values are .
    let mut input_matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let expanded_rows = expand_rows(&mut input_matrix);
    println!("expanded_rows: {:?}", expanded_rows);
    let expanded_columns = expand_columns(&mut input_matrix);
    println!("expanded_columns: {:?}", expanded_columns);

    for (row, line) in input.lines().enumerate() {
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

        let range_x = Range {
            start: min(galaxy_1.position.x, galaxy_2.position.x),
            end: max(galaxy_1.position.x, galaxy_2.position.x),
        };
        let range_y = Range {
            start: min(galaxy_1.position.y, galaxy_2.position.y),
            end: max(galaxy_1.position.y, galaxy_2.position.y),
        };
        // println!("{:?}", galaxy_1);
        // println!("{:?}", galaxy_2);
        // println!("x {:?}", range_x);
        // println!("y {:?}", range_y);

        let mut shortest_distance = delta.x.abs() + delta.y.abs();
        // println!("shortest_distance: {}", shortest_distance);

        for x in expanded_columns.iter() {
            if range_x.contains(x) {
                shortest_distance += multiplier - 1;
            }
        }
        for y in expanded_rows.iter() {
            if range_y.contains(y) {
                shortest_distance += multiplier - 1;
            }
        }

        // println!("new shortest_distance: {}", shortest_distance);
        // println!("---------------");

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
    fn test_part_2() {
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

        let result = part_2(input, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("./input.txt");

        let result = part_2(input, 1000000);

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
