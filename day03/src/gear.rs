use crate::part_nr::PartNr;

#[derive(Debug, PartialEq)]
pub struct Gear {
    part_nr1: PartNr,
    part_nr2: PartNr,
}

impl Gear {
    pub fn new(part_nr1: PartNr, part_nr2: PartNr) -> Self {
        Gear { part_nr1, part_nr2 }
    }

    pub fn ratio(&self) -> u32 {
        self.part_nr2.nr() * self.part_nr1.nr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::Point;

    #[test]
    fn test_ratio() {
        // Arrange
        let gear = Gear::new(
            PartNr::new(3, Point::new(0, 0)),
            PartNr::new(4, Point::new(1, 1)),
        );
        let expected: u32 = 12;

        // Act
        let actual: u32 = gear.ratio();

        // Assert
        assert_eq!(actual, expected);
    }
}
