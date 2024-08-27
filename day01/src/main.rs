use std::str::Lines;
use common;
use common::InputReader;

fn main() {
    let input_reader: InputReader = common::create_input_reader(1);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> u32 {
    lines
        .map(|line| get_calibration_value(line) as u32)
        .sum()
}

fn solve_part2(lines: Lines) -> u32 {
    lines
        .map(|line| get_real_calibration_value(line) as u32)
        .sum()
}

fn get_calibration_value(line: &str) -> u8 {
    let first_pos: Option<usize> = line.find(|c: char| c.is_digit(10));
    let last_pos: Option<usize> = line.rfind(|c: char| c.is_digit(10));
    if first_pos.is_none() || last_pos.is_none() {
        return 0;
    }

    get_digit_value_within_string(line, first_pos.unwrap()) * 10 +
        get_digit_value_within_string(line, last_pos.unwrap())
}

fn get_digit_value_within_string(line: &str, index: usize) -> u8 {
    line.as_bytes()[index] - '0' as u8
}

fn get_real_calibration_value(line: &str) -> u8 {
    let words: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut values: Vec<u8> = Vec::new();
    for i in 0..line.len() {
        if line.chars().nth(i).unwrap().is_digit(10) {
            values.push(line.bytes().nth(i).unwrap() - '0' as u8);
        }

        for word_index in 0..words.len() {
            let word = words[word_index];
            if line[i..].starts_with(word) {
                values.push((word_index + 1) as u8);
            }
        }
    }

    if values.len() == 0 {
        return 0;
    }

    values.first().unwrap() * 10 + values.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calibration_value_outsides() {
        run_get_calibration_value_test_case("1abc2", 12);
    }

    #[test]
    fn test_get_calibration_value_single() {
        //noinspection SpellCheckingInspection
        run_get_calibration_value_test_case("treb7uchet", 77);
    }

    #[test]
    fn test_solve_part1() {
        // Arrange
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let expected: u32 = 142;

        // Act
        let actual: u32 = solve_part1(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2() {
        // Arrange
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let expected: u32 = 281;

        // Act
        let actual: u32 = solve_part2(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_real_calibration_value_unused_digit()
    {
        run_get_real_calibration_value_test_case("two1nine", 29);
    }

    fn run_get_calibration_value_test_case(input: &str, expected: u8) {
        // Arrange

        // Act
        let actual = get_calibration_value(input);

        // Assert
        assert_eq!(actual, expected);
    }

    fn run_get_real_calibration_value_test_case(input: &str, expected: u8) {
        // Arrange

        // Act
        let actual = get_real_calibration_value(input);

        // Assert
        assert_eq!(actual, expected);
    }
}
