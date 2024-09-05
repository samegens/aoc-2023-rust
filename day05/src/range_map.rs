
#[derive(Debug, PartialEq)]
pub struct RangeMap {
    source_start_index: u64,
    source_length: u64,
    dest_start_index: u64
}

impl RangeMap {
    pub fn parse(line: &str) -> Self {
        let numbers: Vec<u64> = line
            .split(' ')
            .map(|number_text| number_text.parse::<u64>().unwrap())
            .collect();

        let dest_start_index = numbers[0];
        let source_start_index = numbers[1];
        let source_length = numbers[2];

        RangeMap { source_start_index, source_length, dest_start_index }
    }

    pub fn contains(&self, n: u64) -> bool {
        n >= self.source_start_index &&
            n < self.source_start_index + self.source_length
    }

    pub fn map(&self, source: u64) -> u64 {
        if self.contains(source) {
            self.dest_start_index + source - self.source_start_index
        }
        else {
            source
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        // Arrange
        let line = "50 98 2";
        let expected = RangeMap { source_start_index: 98, source_length: 2, dest_start_index: 50 };

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

    fn run_map_test_case(source: u64, expected: u64) {
        // Arrange
        let range_map = RangeMap::parse("50 98 2");

        // Act
        let actual = range_map.map(source);

        // Assert
        assert_eq!(actual, expected);
    }
}