use regex::Regex;

use super::part_1::{generate_map_paths, generate_maps};

#[allow(dead_code)]
fn part_2(input: &str) -> i64 {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let seeds = Regex::new(r"\d+\s+\d+")
        .unwrap()
        .find_iter(sections[0])
        .flat_map(|m| {
            let parts = m.as_str().split_whitespace().collect::<Vec<_>>();
            let start = parts[0].parse::<i64>().unwrap();
            let length = parts[1].parse::<i64>().unwrap();
            (start..start + length).collect::<Vec<i64>>() // Use inclusive range and collect to Vec<i64>
        })
        .collect::<Vec<i64>>();

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
