use indexmap::IndexMap;
use rayon::prelude::*;
use regex::Regex;

use super::part_1::{generate_maps, Mapping};

pub fn generate_map_paths_2<T>(seeds: T, all_maps: &IndexMap<&str, Vec<Mapping>>) -> i64
where
    T: IntoIterator<Item = i64>,
{
    let mut lowest_value = i64::MAX;

    for seed in seeds {
        let mut last_value = seed;

        for map in all_maps.keys() {
            let val_from_map = all_maps
                .get(map)
                .unwrap()
                .iter()
                .filter(|m| (m.source_start..m.source_start + m.length).contains(&last_value))
                .map(|m| m.dest_start + (last_value - m.source_start))
                .collect::<Vec<i64>>();

            if let Some(val_from_map) = val_from_map.first() {
                last_value = *val_from_map;
            }
        }
        if last_value < lowest_value {
            lowest_value = last_value;
        }
    }
    lowest_value
}

#[allow(dead_code)]
fn part_2(input: &str) -> i64 {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let seeds = Regex::new(r"\d+\s+\d+")
        .unwrap()
        .find_iter(sections[0])
        .map(|m| {
            let parts = m.as_str().split_whitespace().collect::<Vec<_>>();
            let start = parts[0].parse::<i64>().unwrap();
            let length = parts[1].parse::<i64>().unwrap();
            start..start + length
        })
        .collect::<Vec<_>>();

    // let seed_mins = seeds.iter().map(|s| s.start).collect::<Vec<_>>();
    // let seed_maxs = seeds.iter().map(|s| s.end).collect::<Vec<_>>();

    let all_maps = generate_maps(sections);

    let min_value: i64 = seeds
        .par_iter()
        .map(|seeds_slice| {
            let range = seeds_slice.clone().collect::<Vec<i64>>();
            generate_map_paths_2(range, &all_maps)
        })
        .collect::<Vec<i64>>()
        .iter()
        .min()
        .copied()
        .unwrap();

    min_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_2() {
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

        let result = part_2(input);
        assert_eq!(result, 46);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("./input.txt");

        let result = part_2(input);
        print_green(&format!("Day 5, part 2 result: {}", result));
    }
}
