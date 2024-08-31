use std::str::Lines;
use common::{Grid, Point};
use crate::part_nr::PartNr;

pub struct Engine {
    grid: Grid<char>,
    width: usize,
    height: usize
}

impl Engine {
    pub fn parse(lines: Lines) -> Self {
        let grid: Grid<char> = Grid::parse(lines);
        let width = grid.width();
        let height = grid.height();
        Engine { grid, width , height }
    }

    pub fn get_part_numbers(&self) -> Vec<PartNr> {
        let mut part_numbers: Vec<PartNr> = Vec::new();
        let mut current_number: Option<u32> = None;
        let mut current_pos = Point::new(0, 0);
        let mut is_adjacent_to_symbol = false;
        for y in 0..self.height {
            for x in 0..self.width {
                let ch: &char = self.grid.at(x, y).unwrap();
                if ch.is_digit(10) {
                    if current_number.is_none() {
                        current_number = Some(ch.to_digit(10).unwrap());
                        current_pos = Point::new(x as i64, y as i64);
                        is_adjacent_to_symbol = self.is_adjacent_to_symbol(x, y);
                    }
                    else {
                        let new_number = 10 * current_number.unwrap() +
                            ch.to_digit(10).unwrap();
                        current_number = Some(new_number);
                        is_adjacent_to_symbol = is_adjacent_to_symbol || self.is_adjacent_to_symbol(x, y);
                    }
                }
                else {
                    if let Some(nr) = current_number {
                        if is_adjacent_to_symbol {
                            part_numbers.push(PartNr::new(nr, current_pos));
                        }
                    }
                    current_number = None;
                }
            }
            if let Some(nr) = current_number {
                if is_adjacent_to_symbol {
                    part_numbers.push(PartNr::new(nr, current_pos));
                }
            }
            current_number = None;
        }

        part_numbers
    }

    pub fn is_adjacent_to_symbol(&self, x: usize, y: usize) -> bool {
        for dx in -1i32..=1i32 {
            for dy in -1i32..=1i32 {
                let new_x: i32 = x as i32 + dx;
                let new_y: i32 = y as i32 + dy;
                if (dx != 0 || dy != 0) && new_x >= 0 && new_y >= 0 {
                    if self.is_symbol(new_x as usize, new_y as usize) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn is_symbol(&self, x: usize, y: usize) -> bool {
        let non_symbols = "0123456789.";
        self.grid
            .at(x, y)
            .map_or(false, |ch: &char| !non_symbols.contains(*ch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symbol_with_symbol_at_location() {
        run_is_symbol_test_case(1, 0, true);
    }

    #[test]
    fn test_is_symbol_with_period_at_location() {
        run_is_symbol_test_case(1, 1, false);
    }

    fn run_is_symbol_test_case(x: usize, y: usize, expected: bool) {
        // Arrange
        let engine: Engine = Engine::parse("0*2\n@.^".lines());

        // Act
        let actual = engine.is_symbol(x, y);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_adjacent_to_symbol_with_adjacent_symbol() {
        run_is_adjacent_to_symbol_test_case(2, 1, true);
    }

    #[test]
    fn test_is_adjacent_to_symbol_without_adjacent_symbol() {
        run_is_adjacent_to_symbol_test_case(6, 1, false);
    }

    fn run_is_adjacent_to_symbol_test_case(x: usize, y: usize, expected: bool) {
        // Arrange
        let engine_text = r#"...*......
..35..633.
.........."#;
        let engine: Engine = Engine::parse(engine_text.lines());

        // Act
        let actual = engine.is_adjacent_to_symbol(x, y);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_part_numbers() {
        // Arrange
        let engine_text = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
        let engine = Engine::parse(engine_text.lines());
        let expected: Vec<PartNr> = vec![
            PartNr::new(467, Point::new(0, 0)),
            PartNr::new(35, Point::new(2, 2)),
            PartNr::new(633, Point::new(6, 2)),
            PartNr::new(617, Point::new(0, 4)),
            PartNr::new(592, Point::new(2, 6)),
            PartNr::new(755, Point::new(6, 7)),
            PartNr::new(664, Point::new(1, 9)),
            PartNr::new(598, Point::new(5, 9))];

        // Act
        let actual: Vec<PartNr> = engine.get_part_numbers();

        // Assert
        assert_eq!(actual, expected);
    }
}