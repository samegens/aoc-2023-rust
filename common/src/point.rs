#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    pub fn translate(&self, dx: i64, dy: i64) -> Self {
        Point::new(self.x + dx, self.y + dy)
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    pub fn manhattan_distance(&self, other: &Point) -> u64 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        // Arrange
        let point = Point::new(3, 4);
        let expected = Point::new(5, 7);

        // Act

        let actual = point.translate(2, 3);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_distance_to() {
        // Arrange
        let point1 = Point::new(1, 2);
        let point2 = Point::new(4, 6);
        let expected = 5.0;

        // Act
        let actual = point1.distance_to(&point2);

        // Assert
        assert!((actual - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn test_manhattan_distance() {
        // Arrange
        let point1 = Point::new(1, 2);
        let point2 = Point::new(4, 6);
        let expected = 7;

        // Act
        let actual = point1.manhattan_distance(&point2);

        // Assert
        assert_eq!(actual, expected);
    }
}
