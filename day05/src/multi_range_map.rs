use crate::range_map::RangeMap;

#[derive(Debug, PartialEq)]
pub struct MultiRangeMap {
    range_maps: Vec<RangeMap>
}

impl MultiRangeMap {
    pub fn parse(lines: &Vec<&str>) -> Self {
        let range_maps: Vec<RangeMap> = lines[1..]
            .iter()
            .map(|line| RangeMap::parse(line))
            .collect();
        MultiRangeMap { range_maps }
    }

    pub fn map(&self, source: u64) -> u64 {
        self.range_maps
            .iter()
            .filter(|range_map| range_map.contains(source))
            .nth(0)
            .map_or(source, |range_map| range_map.map(source))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_source_in_first_range() {
        run_map_test_case(99, 51);
    }

    #[test]
    fn test_map_source_in_second_range() {
        run_map_test_case(53, 55);
    }

    #[test]
    fn test_map_source_in_no_range() {
        run_map_test_case(42, 42);
    }

    fn run_map_test_case(source: u64, expected: u64) {
        // Arrange
        let lines = r#"seed-to-soil map:
50 98 2
52 50 48"#;
        let multi_range_map: MultiRangeMap = MultiRangeMap::parse(&lines.lines().collect());

        // Act
        let actual = multi_range_map.map(source);

        // Assert
        assert_eq!(actual, expected);
    }
}