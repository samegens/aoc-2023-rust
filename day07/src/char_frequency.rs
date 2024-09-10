use std::cmp::Ordering;
use crate::card::get_card_value;

#[derive(Debug, PartialEq, Eq)]
pub struct CharFrequency {
    ch: char,
    frequency: u32
}

impl CharFrequency {
    pub fn new(ch: char, frequency: u32) -> CharFrequency {
        CharFrequency { ch, frequency }
    }

    pub fn get_frequency(&self) -> u32 {
        self.frequency
    }
}

impl Ord for CharFrequency {

    /// Implements cmp so higher frequencies will come first.
    fn cmp(&self, other: &Self) -> Ordering {
        let freq_cmp = other.frequency.cmp(&self.frequency);
        if freq_cmp != Ordering::Equal {
            return freq_cmp;
        }

        get_card_value(other.ch).cmp(&get_card_value(self.ch))
    }
}

impl PartialOrd for CharFrequency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp() {
        // Arrange
        let f1 = CharFrequency::new('A', 1);
        let f2 = CharFrequency::new('2', 2);
        let expected = Ordering::Greater;

        // Act
        let actual = f1.cmp(&f2);

        // Assert
        assert_eq!(actual, expected);
    }
}