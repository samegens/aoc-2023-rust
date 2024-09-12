mod hand;
mod char_frequency;
mod card;
mod hand_comparer;
mod hand_categorizer;

use common::InputReader;
use std::str::Lines;
use crate::hand::Hand;
use crate::hand_categorizer::{HandCategorizer, JokerHandCategorizer, RegularHandCategorizer};
use crate::hand_comparer::{HandComparer, JokerHandComparer, RegularHandComparer};

fn main() {
    let input_reader: InputReader = InputReader::new(7);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    let comparer = RegularHandComparer {};
    let categorizer = RegularHandCategorizer {};
    let mut hand_bids: Vec<(Hand, i64)> = lines
        .map(|line| parse_line(line, &comparer, &categorizer))
        .collect();
    hand_bids.sort_by(|a, b| a.0.cmp(&b.0));
    let mut score: i64 = 0;
    for rank in 1..=hand_bids.len() {
        score = score + rank as i64 * hand_bids[rank - 1].1;
    }
    score
}

fn parse_line<'a>(line: &str, comparer: &'a dyn HandComparer, categorizer: &dyn HandCategorizer) -> (Hand<'a>, i64) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let hand = Hand::new(parts[0], comparer, categorizer);
    let bid = parts[1].parse::<i64>().unwrap();
    (hand, bid)
}

fn solve_part2(lines: Lines) -> i64 {
    let comparer = JokerHandComparer {};
    let categorizer = JokerHandCategorizer {};
    let mut hand_bids: Vec<(Hand, i64)> = lines
        .map(|line| parse_line(line, &comparer, &categorizer))
        .collect();
    hand_bids.sort_by(|a, b| a.0.cmp(&b.0));
    let mut score: i64 = 0;
    for rank in 1..=hand_bids.len() {
        score = score + rank as i64 * hand_bids[rank - 1].1;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_solve_part1()
    {
        // Arrange
        let expected: i64 = 6440;

        // Act
        let actual: i64 = solve_part1(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2()
    {
        // Arrange
        let expected: i64 = 5905;

        // Act
        let actual: i64 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }
}
