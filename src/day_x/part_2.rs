#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    let _placeholder = input;
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_2() {
        let input = r#"
"#
        .trim_start_matches('\n');

        let result = part_2(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn run_part_2() {
        // let input = include_str!("./input.txt");
        //
        // let result = part_2(input);
        print_green(&format!("Day X, part 2 result: {}", 0));
    }
}
