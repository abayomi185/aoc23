use std::iter::zip;

pub struct Race {
    pub time: i64,
    pub distance: i64,
}

pub fn get_winning_ways(race: &Race) -> Vec<i64> {
    let time_range = 0..race.time;

    let mut winning_ways = Vec::new();

    time_range.for_each(|time| {
        // Speed is proportional to time held which is also the time value in this case

        let remainder_time = race.time - time;

        let speed = time;
        let dist = speed * remainder_time;

        if dist > race.distance {
            winning_ways.push(speed);
        }
    });

    winning_ways
}

#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    let sections = input.split_once('\n').unwrap();

    let times = sections
        .0
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let distances = sections
        .1
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let races = zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<Race>>();

    races
        .iter()
        .map(|race| {
            // Stop the cap - count
            get_winning_ways(race).len() as i32
        })
        .collect::<Vec<_>>()
        .iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::print_green;

    #[test]
    fn test_part_1() {
        let input = r#"
Time:      7  15   30
Distance:  9  40  200
"#
        .trim_start_matches('\n');

        let result = part_1(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("./input.txt");

        let result = part_1(input);
        print_green(&format!("Day 6, part 1 result: {}", result));
    }
}
