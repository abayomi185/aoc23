use regex::Regex;

#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    let numbers = input.lines().map(|line| {
        let left = Regex::new(r"^[A-Za-z]*\d")
            .unwrap()
            .find(line)
            .unwrap()
            .as_str()
            .chars()
            .last()
            .unwrap();
        let right = Regex::new(r"\d[A-Za-z]*$")
            .unwrap()
            .find(line)
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap();
        let value = &format!("{}{}", left, right);
        value.parse::<i32>().unwrap()
    });

    println!("numbers: {:?}", numbers.clone().collect::<Vec<i32>>());

    numbers.sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_1() {
        let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#
        .trim_start_matches('\n');

        let result = part_1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("./input.txt");

        let result = part_1(input);
        print_green(&format!("Day 1, part 1 result: {result}"));
    }
}
