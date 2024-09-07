#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range<T>
where
    T: PartialOrd + Copy,
{
    start: T,
    length: T,
}

impl<T> Range<T>
where
    T: PartialOrd + Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    pub fn new(start: T, length: T) -> Self {
        Range { start, length }
    }

    pub fn start(&self) -> T {
        self.start
    }

    pub fn end(&self) -> T {
        self.start + self.length
    }

    /// Check if this range overlaps with another range.
    pub fn overlaps(&self, other: &Range<T>) -> bool {
        self.start < other.end() && other.start < self.end()
    }

    /// Split two ranges into parts: non-overlapping and overlapping.
    /// Returns a single vector containing the non-overlapping and overlapping parts.
    pub fn split(&self, other: &Range<T>) -> Vec<Range<T>> {
        let mut result = Vec::new();

        if !self.overlaps(other) {
            // No overlap, just return both ranges as is.
            result.push(*self);
            result.push(*other);
            return result;
        }

        let overlap_start = if self.start > other.start { self.start } else { other.start };
        let overlap_end = if self.end() < other.end() { self.end() } else { other.end() };
        let overlap_length = overlap_end - overlap_start;
        let overlap_range = Range::new(overlap_start, overlap_length);

        // Non-overlapping part before the overlap in self
        if self.start < overlap_start {
            result.push(Range::new(self.start, overlap_start - self.start));
        }

        // Non-overlapping part before the overlap in other
        if other.start < overlap_start {
            result.push(Range::new(other.start, overlap_start - other.start));
        }

        // Add the overlapping range
        result.push(overlap_range);

        // Non-overlapping part after the overlap in self
        if overlap_end < self.end() {
            result.push(Range::new(overlap_end, self.end() - overlap_end));
        }

        // Non-overlapping part after the overlap in other
        if overlap_end < other.end() {
            result.push(Range::new(overlap_end, other.end() - overlap_end));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_no_overlap() {
        // Arrange
        let range1 = Range::new(10, 5);
        let range2 = Range::new(20, 5);

        // Act
        let actual = range1.split(&range2);

        // Assert
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], range1);
        assert_eq!(actual[1], range2);
    }

    #[test]
    fn test_split_full_overlap() {
        // Arrange
        let range1 = Range::new(10, 10); // Range from 10 to 20
        let range2 = Range::new(12, 5);  // Range from 12 to 17 (fully inside range1)

        // Act
        let actual = range1.split(&range2);

        // Assert
        assert_eq!(actual.len(), 3);
        assert_eq!(actual[0], Range::new(10, 2));  // Non-overlapping part of range1 (before overlap)
        assert_eq!(actual[1], Range::new(12, 5));  // Overlapping part
        assert_eq!(actual[2], Range::new(17, 3));  // Non-overlapping part of range1 (after overlap)
    }

    #[test]
    fn test_split_partial_overlap() {
        // Arrange
        let range1 = Range::new(10, 10);
        let range2 = Range::new(15, 10);

        // Act
        let actual = range1.split(&range2);

        // Assert
        assert_eq!(actual.len(), 3);
        assert_eq!(actual[0], Range::new(10, 5));
        assert_eq!(actual[1], Range::new(15, 5));
        assert_eq!(actual[2], Range::new(20, 5));
    }

    #[test]
    fn test_split_exact_overlap() {
        // Arrange
        let range1 = Range::new(10, 10);
        let range2 = Range::new(10, 10);

        // Act
        let actual = range1.split(&range2);

        // Assert
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], Range::new(10, 10));
    }

    #[test]
    fn test_split_non_overlapping_touching() {
        // Arrange
        let range1 = Range::new(10, 10);
        let range2 = Range::new(20, 10);

        // Act
        let actual = range1.split(&range2);

        // Assert
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], range1);
        assert_eq!(actual[1], range2);
    }
}
