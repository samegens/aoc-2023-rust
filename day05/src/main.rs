use std::str::Lines;
use common::{split_into_blocks, InputReader};
use crate::multi_range_map::MultiRangeMap;

mod range_map;
mod multi_range_map;

fn main() {
    let input_reader: InputReader = InputReader::new(5);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> u64 {
    let blocks: Vec<Vec<&str>> = split_into_blocks(lines);
    let seeds: Vec<u64> = blocks[0][0]
        .split_whitespace()
        .skip(1) // Skip the "seeds:" part
        .map(|s| s.parse().unwrap()) // Parse each number into u64
        .collect();
    let multi_range_maps: Vec<MultiRangeMap> = blocks[1..]
        .iter()
        .map(|block| MultiRangeMap::parse(block))
        .collect();
    seeds.into_iter()
        .map(|seed| multi_range_maps.iter()
        .fold(seed, |acc, multi_range_map| multi_range_map.map(acc)))
        .min()
        .unwrap()
}

fn solve_part2(lines: Lines) -> u64 {
    // Engine::parse(lines)
    //     .get_gears()
    //     .iter()
    //     .map(|gear| gear.ratio())
    //     .sum()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn test_solve_part1()
    {
        // Arrange
        let expected: u64 = 35;

        // Act
        let actual: u64 = solve_part1(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    // #[test]
    // fn test_solve_part2()
    // {
    //     // Arrange
    //     let expected: u32 = 467835;
    //
    //     // Act
    //     let actual: u32 = solve_part2(INPUT.lines());
    //
    //     // Assert
    //     assert_eq!(actual, expected);
    // }
}
