use std::collections::HashMap;

use regex::Regex;

#[allow(dead_code)]
fn part_1(input: &str) -> i64 {
    let arrangements: Vec<i64> = Vec::new();

    for line in input.lines() {
        let contiguous_unknown = Regex::new(r"\?+[\s\.]")
            .unwrap()
            .find_iter(line)
            .map(|m| m.as_str())
            .collect::<Vec<&str>>();

        let paired_unknown = Regex::new(r"\?#+")
            .unwrap()
            .find_iter(line)
            .map(|m| m.as_str())
            .collect::<Vec<&str>>();

        for (index, char) in line.chars().enumerate() {
            if char == '#' {
                // println!("index: {}", index);
            }
            if char == '?' {
                // println!("index: {}", index);
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;
    use std::path::Path;

    #[test]
    fn test_part_1() {
        let input = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#
        .trim_start_matches('\n');

        let result = part_1(input);
        assert_eq!(result, 0);
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
