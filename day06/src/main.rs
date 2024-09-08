mod race;

use crate::race::Race;
use common::{parse_numbers_from_string, InputReader};
use std::str::Lines;

fn main() {
    let input_reader: InputReader = InputReader::new(6);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    parse_races_info(lines).iter()
        .map(|race| race.get_nr_options_that_beat_record())
        .fold(1, |acc, dist| acc * dist)
}

fn solve_part2(_: Lines) -> i64 {
    0
}

fn parse_races_info(lines: Lines) -> Vec<Race> {
    // Time:      7  15   30
    // Distance:  9  40  200
    let mut lines = lines.peekable();
    let durations_text: &str = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1).unwrap()
        .trim();
    let durations: Vec<i64> = parse_numbers_from_string(durations_text);
    let records_text: &str = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1).unwrap()
        .trim();
    let records: Vec<i64> = parse_numbers_from_string(records_text);
    assert_eq!(durations.len(), records.len());

    let mut races: Vec<Race> = Vec::with_capacity(durations.len());
    for i in 0..durations.len() {
        races.push(Race::new(durations[i], records[i]));
    }
    races
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_solve_part1()
    {
        // Arrange
        let expected: i64 = 288;

        // Act
        let actual: i64 = solve_part1(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    // #[test]
    // fn test_solve_part2()
    // {
    //     // Arrange
    //     let expected: i64 = 46;
    //
    //     // Act
    //     let actual: i64 = solve_part2(INPUT.lines());
    //
    //     // Assert
    //     assert_eq!(actual, expected);
    // }
}
