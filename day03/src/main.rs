mod engine;

use std::str::Lines;
use common;
use common::InputReader;
use crate::engine::Engine;

fn main() {
    let input_reader: InputReader = InputReader::new(3);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> u32 {
    Engine::parse(lines)
        .get_part_numbers()
        .iter()
        .sum()
}

fn solve_part2(_: Lines) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1()
    {
        // Arrange
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let expected: u32 = 4361;

        // Act
        let actual: u32 = solve_part1(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }
}
