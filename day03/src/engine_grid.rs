use std::str::Lines;
use common::Grid;

pub struct EngineGrid {
    grid: Grid<char>
}

impl EngineGrid {
    pub fn parse(lines: Lines) -> Self {
        let grid = Grid::parse(lines);
        EngineGrid { grid }
    }

    pub fn width(&self) -> usize {
        self.grid.width()
    }

    pub fn height(&self) -> usize {
        self.grid.height()
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&char> {
        self.grid.at(x, y)
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

    pub fn is_gear_symbol(&self, x: usize, y: usize) -> bool {
        self.grid
            .at(x, y)
            .map_or(false, |ch: &char| *ch == '*')
    }
}
