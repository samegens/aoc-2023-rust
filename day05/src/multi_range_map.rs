use common::Range;
use crate::range_map::RangeMap;

/// MultiRange map is a map that is able to map numbers and ranges from multiple disjoint ranges
/// to new numbers and ranges.
#[derive(Debug, PartialEq)]
pub struct MultiRangeMap {
    range_maps: Vec<RangeMap>
}

impl MultiRangeMap {
    pub fn parse(lines: &Vec<&str>) -> Self {
        let mut range_maps: Vec<RangeMap> = lines[1..]
            .iter()
            .map(|line| RangeMap::parse(line))
            .collect();

        // Sort the maps on source so the implementation of map_range can be simpler.
        range_maps.sort_by(|a, b| a.source().start().cmp(&b.source().start()));

        MultiRangeMap { range_maps }
    }

    pub fn map(&self, source: i64) -> i64 {
        self.range_maps
            .iter()
            .filter(|range_map| range_map.contains(source))
            .nth(0)
            .map_or(source, |range_map| range_map.map(source))
    }

    pub fn map_range(&self, source: Range<i64>) -> Vec<Range<i64>> {
        let mut result: Vec<Range<i64>> = Vec::new();

        // Start with the full `self` range
        let mut current_source_range = source;

        for range_map in self.range_maps.iter() {
            // If there's no overlap, continue to the next range
            if !current_source_range.overlaps(&range_map.source()) {
                continue;
            }

            // Calculate the overlap
            let overlap_start = if current_source_range.start() > range_map.source().start() {
                current_source_range.start()
            } else {
                range_map.source().start()
            };
            let overlap_end = if current_source_range.end() < range_map.source().end() {
                current_source_range.end()
            } else {
                range_map.source().end()
            };

            // Add the non-overlapping part before the overlap (if any).
            // Since there's no overlap, this part doesn't need to be mapped.
            if current_source_range.start() < overlap_start {
                result.push(Range::new(current_source_range.start(),
                                       overlap_start - current_source_range.start()));
            }

            // Map and add the overlapping part
            let overlapping_range = Range::new(overlap_start, overlap_end - overlap_start);
            let map_result= range_map.map_range(overlapping_range);
            assert_eq!(map_result.len(), 1);
            result.push(map_result[0]);

            // Update the current range to the remaining part after the overlap
            current_source_range = Range::new(overlap_end, current_source_range.end() - overlap_end);
        }

        // Add the remaining part of `self` after processing all ranges, this is not overlapping
        // with any range map, so we don't need to map.
        if current_source_range.length() > 0 {
            result.push(current_source_range);
        }

        result
    }

    pub fn map_ranges(&self, ranges_to_map: Vec<Range<i64>>) -> Vec<Range<i64>> {
        ranges_to_map.iter()
            .map(|range_to_map| self.map_range(*range_to_map))
            .flatten()
            .collect()
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

    fn run_map_test_case(source: i64, expected: i64) {
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

    #[test]
    fn test_map_range() {
        // Arrange
        let range_to_map = Range::new(74, 14);
        let multi_range_map_text = r#"light-to-temperature map:
45 77 23
81 45 19
68 64 13"#;
        let multi_range_map = MultiRangeMap::parse(&multi_range_map_text.lines().collect());
        let expected = vec![
            Range::new(78, 3),
            Range::new(45, 11)
        ];

        // Act
        let actual = multi_range_map.map_range(range_to_map);

        // Assert
        assert_eq!(actual, expected);
    }
}