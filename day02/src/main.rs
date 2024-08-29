mod cube_color;

mod game_set;
mod game;

use crate::game::Game;
use common;
use common::InputReader;
use std::str::{Lines};

fn main() {
    let input_reader: InputReader = common::create_input_reader(2);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> u32 {
    lines
        .map(|line| Game::parse(line))
        .filter(|game| game.is_possible_part1())
        .map(|game| game.id().to_u32())
        .sum()
}

fn solve_part2(lines: Lines) -> u32 {
    lines
        .map(|line| Game::parse(line).power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1()
    {
        // Arrange
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let expected: u32 = 8;

        // Act
        let actual: u32 = solve_part1(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2() {
        // Arrange
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let expected: u32 = 2286;

        // Act
        let actual: u32 = solve_part2(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }
}