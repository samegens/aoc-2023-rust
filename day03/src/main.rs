mod engine;
mod part_nr;
mod gear;
mod engine_grid;

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
        .part_numbers()
        .iter()
        .map(|part_nr| part_nr.nr())
        .sum()
}

fn solve_part2(lines: Lines) -> u32 {
    Engine::parse(lines)
        .get_gears()
        .iter()
        .map(|gear| gear.ratio())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_solve_part1()
    {
        // Arrange
        let expected: u32 = 4361;

        // Act
        let actual: u32 = solve_part1(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2()
    {
        // Arrange
        let expected: u32 = 467835;

        // Act
        let actual: u32 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }
}
