use super::part_1::create_hash_map;

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    let map = create_hash_map(input);

    map.values()
        .map(|game_map| {
            let max_red = *game_map
                .get("red")
                .and_then(|vec| vec.iter().max())
                .unwrap();
            let max_green = *game_map
                .get("green")
                .and_then(|vec| vec.iter().max())
                .unwrap();
            let max_blue = *game_map
                .get("blue")
                .and_then(|vec| vec.iter().max())
                .unwrap();
            max_red * max_green * max_blue
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_2() {
        let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#
        .trim_start_matches('\n');

        let result = part_2(input);
        assert_eq!(result, 2286);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("./input.txt");

        let result = part_2(input);
        print_green(&format!("Day 2, part 2 result: {}", result));
    }
}
