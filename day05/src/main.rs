use std::str::Lines;
use common::{split_into_blocks, InputReader, Range};
use crate::multi_range_map::MultiRangeMap;

mod range_map;
mod multi_range_map;

fn main() {
    let input_reader: InputReader = InputReader::new(5);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    let blocks: Vec<Vec<&str>> = split_into_blocks(lines);
    let seeds: Vec<i64> = parse_seeds_line(blocks[0][0]);
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

fn solve_part2(lines: Lines) -> i64 {
    let blocks: Vec<Vec<&str>> = split_into_blocks(lines);
    let seed_numbers: Vec<i64> = parse_seeds_line(blocks[0][0]);
    let seed_ranges: Vec<Range<i64>> = seed_numbers
        .chunks(2)
        .map(|chunk| Range::new(chunk[0], chunk[1]))
        .collect();
    let multi_range_maps: Vec<MultiRangeMap> = blocks[1..]
        .iter()
        .map(|block| MultiRangeMap::parse(block))
        .collect();
    seed_ranges.into_iter()
        .map(|seed_range| map_from_seed_to_location(seed_range, &multi_range_maps))
        .map(|ranges| get_nearest_location(ranges))
        .min()
        .unwrap()
}

fn get_nearest_location(ranges: Vec<Range<i64>>) -> i64 {
    ranges.iter()
        .map(|range| range.start())
        .min()
        .unwrap()
}

fn map_from_seed_to_location(seed_range: Range<i64>, multi_range_maps: &Vec<MultiRangeMap>) -> Vec<Range<i64>> {
    multi_range_maps.iter()
        .fold(vec![seed_range],
              |acc, multi_range_map| multi_range_map.map_ranges(acc))
}

pub fn parse_seeds_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .skip(1) // Skip the "seeds:" part
        .map(|s| s.parse().unwrap()) // Parse each number into i64
        .collect()
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
        let expected: i64 = 35;

        // Act
        let actual: i64 = solve_part1(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2()
    {
        // Arrange
        let expected: i64 = 46;

        // Act
        let actual: i64 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }
}
