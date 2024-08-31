use crate::engine_grid::EngineGrid;
use crate::gear::Gear;
use crate::part_nr::PartNr;
use common::Point;
use indexmap::IndexSet;
use std::str::Lines;

pub struct Engine {
    grid: EngineGrid,
    part_nrs: Vec<PartNr>
}

impl Engine {
    pub fn parse(lines: Lines) -> Self {
        let grid = EngineGrid::parse(lines);
        let part_nrs = Self::get_part_numbers(&grid);
        Engine { grid, part_nrs  }
    }

    pub fn part_numbers(&self) -> &Vec<PartNr> {
        &self.part_nrs
    }

    fn get_part_numbers(grid: &EngineGrid) -> Vec<PartNr> {
        let mut part_numbers: Vec<PartNr> = Vec::new();
        let mut current_number: Option<u32> = None;
        let mut current_pos = Point::new(0, 0);
        let mut is_adjacent_to_symbol = false;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let ch: &char = grid.at(x, y).unwrap();
                if ch.is_digit(10) {
                    if current_number.is_none() {
                        current_number = Some(ch.to_digit(10).unwrap());
                        current_pos = Point::new(x as i64, y as i64);
                        is_adjacent_to_symbol = grid.is_adjacent_to_symbol(x, y);
                    }
                    else {
                        let new_number = 10 * current_number.unwrap() +
                            ch.to_digit(10).unwrap();
                        current_number = Some(new_number);
                        is_adjacent_to_symbol = is_adjacent_to_symbol || grid.is_adjacent_to_symbol(x, y);
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

    pub fn get_gears(&self) -> Vec<Gear> {
        let mut gears: Vec<Gear> = Vec::new();
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                if let Some(gear) = self.is_gear_at(x, y) {
                    gears.push(gear);
                }
            }
        }
        gears
    }

    fn is_gear_at(&self, x: usize, y: usize) -> Option<Gear> {
        if self.grid.is_gear_symbol(x, y) {
            let part_nrs: Vec<PartNr> = self.get_part_numbers_adjacent_at(x, y);
            if part_nrs.len() == 2 {
                return Some(Gear::new(part_nrs[0], part_nrs[1]));
            }
        }

        None
    }

    fn get_part_numbers_adjacent_at(&self, x: usize, y: usize) -> Vec<PartNr> {
        // Using IndexSet (from crate 'indexset') to maintain insertion order,
        // makes it a bit easier to test the code and make it deterministic.
        let mut part_numbers: IndexSet<PartNr> = IndexSet::new();

        for adjacent_point in Point::new(x as i64, y as i64).adjacent_points() {
            if let Some(part_number) = self.get_part_number_at(adjacent_point) {
                part_numbers.insert(part_number);
            }
        }

        part_numbers.iter().cloned().collect()
    }

    fn get_part_number_at(&self, p: Point) -> Option<PartNr> {
        for part_number in self.part_nrs.iter() {
            if part_number.contains_point(p) {
                return Some(*part_number)
            }
        }

        None
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
        let engine: EngineGrid = EngineGrid::parse("0*2\n@.^".lines());

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
        let engine: EngineGrid = EngineGrid::parse(engine_text.lines());

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
        let actual: &Vec<PartNr> = engine.part_numbers();

        // Assert
        assert_eq!(*actual, expected);
    }

    #[test]
    fn test_get_gears() {
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
        let expected: Vec<Gear> = vec![
            Gear::new(PartNr::new(467, Point::new(0, 0)),
                      PartNr::new(35, Point::new(2, 2))),
            Gear::new(PartNr::new(755, Point::new(6, 7)),
                      PartNr::new(598, Point::new(5, 9)))
        ];

        // Act
        let actual: Vec<Gear> = engine.get_gears();

        // Assert
        assert_eq!(actual, expected);
    }
}