use std::{cmp::Ordering, collections::HashMap, iter::zip};

pub fn get_total_winnings(
    input: &str,
    // card_label_map: HashMap<char, i32>, // Old implementation
    card_labels: Vec<char>,
    joker_privilege: bool,
) -> i32 {
    let hand_type_map = HashMap::from([
        ("11111", 1), // High card
        ("2111", 2),  // One pair
        ("221", 3),   // Two pair
        ("311", 4),   // Three of a kind
        ("32", 5),    // Full house
        ("41", 6),    // Four of a kind
        ("5", 7),     // Five of a kind
    ]);

    let mut cards = input
        .lines()
        .map(|line| {
            let hand = line.split_once(' ').unwrap();
            let bid = hand.1.parse::<i32>().unwrap();

            let mut char_count = HashMap::new();
            let mut card_values = Vec::new();

            let mut joker = 0;

            for char in hand.0.chars() {
                if joker_privilege && char == 'J' {
                    joker += 1;
                } else {
                    char_count.insert(char, char_count.get(&char).unwrap_or(&0) + 1);
                }

                // card_values.push(card_label_map.get(&char).unwrap());
                card_values.push(card_labels.iter().position(|&x| x == char).unwrap());
            }

            let mut char_values = char_count.values().cloned().collect::<Vec<_>>();
            char_values.sort_unstable_by(|a, b| b.cmp(a));

            if joker_privilege {
                if char_values.is_empty() {
                    char_values.push(0);
                }
                // println!("{:?}", line);
                // println!("{:?}", char_values);
                char_values[0] += joker;
            }

            // println!("{:?}", char_values);

            let binding = char_values
                .iter()
                .map(|count| count.to_string())
                .collect::<Vec<_>>()
                .join("");
            let char_values_as_string = binding.as_str();

            let hand_type = hand_type_map.get(char_values_as_string).unwrap();

            (hand_type, bid, card_values) // Returning a tuple
        })
        .collect::<Vec<_>>();

    cards.sort_by(|a, b| {
        if a.0 == b.0 {
            zip(a.2.iter(), b.2.iter())
                .find_map(|(x, y)| if x != y { Some(x.cmp(y)) } else { None })
                .unwrap_or(Ordering::Equal)
        } else {
            // Ordering::Greater // alternative
            a.0.cmp(b.0)
        }
    });

    cards
        .iter()
        .enumerate()
        .map(|(index, card)| card.1 * ((index as i32) + 1))
        .sum() // Summing up the bids
}

#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    // let card_label_map = HashMap::from([
    //     ('2', 1),
    //     ('3', 2),
    //     ('4', 3),
    //     ('5', 4),
    //     ('6', 5),
    //     ('7', 6),
    //     ('8', 7),
    //     ('9', 8),
    //     ('T', 9),
    //     ('J', 10),
    //     ('Q', 11),
    //     ('K', 12),
    //     ('A', 13),
    // ]);

    let card_labels = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ]
    .to_vec();

    get_total_winnings(input, card_labels, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;
    use std::path::Path;

    #[test]
    fn test_part_1() {
        let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#
        .trim_start_matches('\n');

        let result = part_1(input);
        assert_eq!(result, 6440);
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
