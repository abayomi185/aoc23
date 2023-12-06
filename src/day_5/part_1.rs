// dest_start   source_start    length
// 50           98              2

use std::collections::HashSet;

use indexmap::IndexMap;
use rayon::prelude::*;
use regex::Regex;

#[derive(Clone)]
pub struct Mapping {
    pub dest_start: i64,
    pub source_start: i64,
    pub length: i64,
}

pub fn generate_maps(sections: Vec<&str>) -> IndexMap<&str, Vec<Mapping>> {
    let mut all_maps: IndexMap<&str, Vec<Mapping>> = IndexMap::new();

    for section in &sections[1..] {
        let map_title = section
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>()[0];

        let map = section
            .lines()
            .skip(1)
            .map(|m| {
                let map_values = Regex::new(r"\d+")
                    .unwrap()
                    .find_iter(m)
                    .map(|m| m.as_str().parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();

                Mapping {
                    dest_start: map_values[0],
                    source_start: map_values[1],
                    length: map_values[2],
                }
            })
            .collect::<Vec<Mapping>>();

        all_maps.insert(map_title, map);
    }
    all_maps
}

pub fn generate_map_paths<T>(seeds: T, all_maps: IndexMap<&str, Vec<Mapping>>) -> Vec<Vec<i64>>
where
    T: IntoParallelIterator<Item = i64>,
{
    seeds
        .into_par_iter() // Changed to parallel iterator
        .map(|seed| {
            let mut seed_map_path: Vec<i64> = Vec::from([seed]);

            for map in all_maps.keys() {
                if let Some(last_val) = seed_map_path.last() {
                    let val_from_map = all_maps
                        .get(map)
                        .unwrap()
                        .iter()
                        .filter(|m| (m.source_start..m.source_start + m.length).contains(last_val))
                        .map(|m| m.dest_start + (last_val - m.source_start))
                        .next(); // We only care about the first value

                    if let Some(val_from_map) = val_from_map {
                        seed_map_path.push(val_from_map);
                    }
                }
            }
            seed_map_path
        })
        .collect() // This will collect the results into a Vec<Vec<i64>>
}

#[allow(dead_code)]
fn part_1(input: &str) -> i64 {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let seeds = Regex::new(r"\d+")
        .unwrap()
        .find_iter(sections[0])
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .collect::<HashSet<i64>>();

    let all_maps = generate_maps(sections);

    let all_seed_map_path = generate_map_paths(seeds, all_maps);

    all_seed_map_path
        .iter()
        .map(|seed_map_path| seed_map_path.last().unwrap())
        .min()
        .copied()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_1() {
        let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#
        .trim_start_matches('\n');

        let result = part_1(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("./input.txt");

        let result = part_1(input);
        print_green(&format!("Day 5, part 1 result: {}", result));
    }
}
