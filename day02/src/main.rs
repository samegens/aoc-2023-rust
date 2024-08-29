mod cube_color;
use cube_color::CubeColor;

mod game_set;
use game_set::GameSet;

use std::collections::HashMap;
use std::str::{FromStr, Lines};
use common;
use common::InputReader;

fn main() {
    let input_reader: InputReader = common::create_input_reader(1);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> u32 {
    // lines
    //     .map(|line| get_calibration_value(line) as u32)
    //     .sum()
    0
}

fn solve_part2(lines: Lines) -> u32 {
    // lines
    //     .map(|line| get_real_calibration_value(line) as u32)
    //     .sum()
    0
}

struct Game(Vec<GameSet>);
