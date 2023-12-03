use std::collections::HashMap;

pub fn create_hash_map(input: &str) -> HashMap<i32, HashMap<&str, Vec<i32>>> {
    let mut map: HashMap<i32, HashMap<&str, Vec<i32>>> = HashMap::new();

    for line in input.lines() {
        let colon_split = line.split(':').collect::<Vec<&str>>();
        let game_id_str = colon_split[0].split_whitespace().collect::<Vec<&str>>()[1];
        let game_id = game_id_str.parse::<i32>().unwrap();

        map.insert(game_id, HashMap::new());

        let sets_split = colon_split[1].split(';').collect::<Vec<&str>>();

        for set in sets_split.into_iter() {
            let comma_split = set.trim().split(',').collect::<Vec<&str>>();

            for item in comma_split.into_iter() {
                let item_split = item.split_whitespace().collect::<Vec<&str>>();
                let count = item_split[0].parse::<i32>().unwrap();
                let color = item_split[1];

                let game_inner_map = map.get_mut(&game_id).unwrap();

                let game_possible = game_inner_map.entry(color).or_insert_with(Vec::new);
                game_possible.push(count);
            }
        }
    }
    map
}

#[allow(dead_code)]
fn part_1(input: &str, case: [&i32; 3]) -> i32 {
    let map = create_hash_map(input);

    map.iter()
        .filter(|(_, game_map)| {
            game_map.get("red").unwrap().iter().all(|&v| v <= *case[0])
                && game_map
                    .get("green")
                    .unwrap()
                    .iter()
                    .all(|&v| v <= *case[1])
                && game_map.get("blue").unwrap().iter().all(|&v| v <= *case[2])
        })
        .map(|(game_id, _)| *game_id)
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_1() {
        let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#
        .trim_start_matches('\n');

        let result = part_1(input, [&12, &13, &14]);
        assert_eq!(result, 8);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("./input.txt");

        let result = part_1(input, [&12, &13, &14]);
        print_green(&format!("Day 2, part 1 result: {}", result));
    }
}
