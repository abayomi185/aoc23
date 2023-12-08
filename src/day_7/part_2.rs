use super::part_1::get_total_winnings;

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    let card_labels = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ]
    .to_vec();

    get_total_winnings(input, card_labels, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;
    use std::path::Path;

    #[test]
    fn test_part_2() {
        let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#
        .trim_start_matches('\n');

        let result = part_2(input);
        assert_eq!(result, 5905);
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
