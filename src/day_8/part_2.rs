use std::collections::{HashMap, VecDeque};

use super::part_1::MapNode;

/*
greatest common divisor
*/
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/*
least common multiple
*/
fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

#[allow(dead_code)]
fn part_2(input: &str) -> i64 {
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

    let mut nodes = network_map
        .keys()
        .cloned()
        .filter(|key| key.ends_with('A'))
        .collect::<VecDeque<_>>();
    let initial_node_length = nodes.len();

    // println!("{:?}", initial_node_length);

    let mut steps = vec![0; initial_node_length];
    let mut steps_flag = vec![false; initial_node_length];

    while !instructions.is_empty() {
        let nodes_to_check = nodes
            .iter()
            .take(initial_node_length)
            .cloned()
            .collect::<Vec<_>>();

        println!("{:?}", nodes_to_check);

        for (index, &node) in nodes_to_check.iter().enumerate() {
            if steps_flag[index] {
                continue;
            }
            if node.ends_with('Z') {
                steps_flag[index] = true;
            } else {
                steps[index] += 1;
            }
        }

        if steps_flag.iter().all(|&x| x) {
            break;
        }

        let direction: char = instructions.pop_front().unwrap();
        instructions.push_back(direction);

        for node in nodes_to_check.iter().rev() {
            if direction == 'L' {
                nodes.push_front(network_map.get(node).unwrap().left);
            } else {
                nodes.push_front(network_map.get(node).unwrap().right);
            }
        }

        nodes.truncate(initial_node_length);
    }

    println!("{:?}", steps);

    steps.iter().fold(1_i64, |acc, &num| lcm(acc, num))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;
    use std::path::Path;

    #[test]
    fn test_part_2() {
        let input = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#
        .trim_start_matches('\n');

        let result = part_2(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("./input.txt");

        let result = part_2(input);

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
