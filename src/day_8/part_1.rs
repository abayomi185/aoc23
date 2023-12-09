use std::collections::{HashMap, VecDeque};

pub struct MapNode<'a> {
    pub left: &'a str,
    pub right: &'a str,
}

#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    let mut network_map: HashMap<&str, MapNode> = HashMap::new();

    for line in input.lines().skip(2) {
        let mut split = line.split(" = ");
        let key = split.next().unwrap();
        let value = split.next().unwrap();

        let directions = value
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ")
            .collect::<Vec<_>>();

        network_map.insert(
            key,
            MapNode {
                left: directions[0],
                right: directions[1],
            },
        );
    }

    let mut instructions = input
        .lines()
        .next()
        .unwrap()
        .trim_start_matches('\n')
        .chars()
        .collect::<VecDeque<_>>();

    // go_to_direction(&network_map, "AAA", instructions, 0).1

    let mut node = "AAA";
    let mut steps = 0;

    while !instructions.is_empty() {
        if node == "ZZZ" {
            break;
        }

        let direction: char = instructions.pop_front().unwrap();
        instructions.push_back(direction);
        if direction == 'L' {
            node = network_map.get(node).unwrap().left;
        } else {
            node = network_map.get(node).unwrap().right;
        }
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;
    use std::path::Path;

    #[test]
    fn test_part_1() {
        let input_1 = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#
        .trim_start_matches('\n');

        let input_2 = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#
        .trim_start_matches('\n');

        let result_1 = part_1(input_1);
        assert_eq!(result_1, 2);

        let result_2 = part_1(input_2);
        assert_eq!(result_2, 6);
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
