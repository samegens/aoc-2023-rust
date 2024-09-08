use std::fmt;
use common::Range;

/// RangeMap maps a source number to a destination number:
/// if the source number falls within the range, it is mapped to the respective
/// destination number,
/// if the source number falls outside the range, it is unchanged.
#[derive(PartialEq)]
pub struct RangeMap {
    source: Range<i64>,
    dest_start_index: i64
}

impl fmt::Debug for RangeMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}->{:?}", self.source, self.dest_start_index)
    }
}

impl RangeMap {
    pub fn parse(line: &str) -> Self {
        let numbers: Vec<i64> = line
            .split(' ')
            .map(|number_text| number_text.parse::<i64>().unwrap())
            .collect();

        let dest_start_index = numbers[0];
        let source_start_index = numbers[1];
        let source_length = numbers[2];

        let source = Range::new(source_start_index, source_length);
        RangeMap { source, dest_start_index }
    }

    pub fn source(&self) -> Range<i64> {
        self.source
    }

    pub fn contains(&self, n: i64) -> bool {
        self.source.contains(n)
    }

    pub fn map(&self, source: i64) -> i64 {
        if self.contains(source) {
            self.dest_start_index + source - self.source.start()
        }
        else {
            source
        }
    }

    pub fn map_range(&self, source: Range<i64>) -> Vec<Range<i64>> {
        let ranges: Vec<Range<i64>> = source.split(&self.source);
        ranges
            .iter()
            .map(|range| if self.source.overlaps(range) { range.shifted(self.delta()) } else { *range })
            .collect()
    }

    fn delta(&self) -> i64 {
        self.dest_start_index as i64 - self.source.start() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        // Arrange
        let line = "50 98 2";
        let source = Range::new(98, 2);
        let expected = RangeMap { source, dest_start_index: 50 };

        // Act
        let actual = RangeMap::parse(line);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_map_within_range() {
        run_map_test_case(99, 51);
    }

    #[test]
    fn test_map_outside_range() {
        run_map_test_case(50, 50);
    }

    fn run_map_test_case(source: i64, expected: i64) {
        // Arrange
        let range_map = RangeMap::parse("50 98 2");

        // Act
        let actual = range_map.map(source);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_map_range_within_range() {
        run_map_range_test_case(98, 2, vec![Range::new(50, 2)]);
    }

    #[test]
    fn test_map_range_outside_range() {
        run_map_range_test_case(96, 2, vec![Range::new(96, 2)]);
    }

    #[test]
    fn test_map_range_fully_overlapping_range() {
        let expected = vec![
            Range::new(97, 1),
            Range::new(50, 2),
            Range::new(100, 1)
        ];
        run_map_range_test_case(97, 4, expected);
    }

    #[test]
    fn test_map_range_partially_overlapping_range_at_end() {
        let expected = vec![
            Range::new(51, 1),
            Range::new(100, 3)
        ];
        run_map_range_test_case(99, 4, expected);
    }

    #[test]
    fn test_map_range_partially_overlapping_range_at_begin() {
        let expected = vec![
            Range::new(96, 2),
            Range::new(50, 1),
        ];
        run_map_range_test_case(96, 3, expected);
    }

    fn run_map_range_test_case(source_start: i64, source_length: i64, expected: Vec<Range<i64>>) {
        // Arrange
        let range_map = RangeMap::parse("50 98 2");
        let source = Range::new(source_start, source_length);

        // Act
        let actual = range_map.map_range(source);

        // Assert
        assert_eq!(actual, expected);
    }
}