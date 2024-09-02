use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Scratchcard {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>
}

impl Scratchcard {
    pub fn parse(line: &str) -> Self {
        let parts: Vec<&str> = line.split("|").collect();
        let winning_numbers_parts: Vec<&str> = parts[0].split(":").collect();
        let winning_numbers: Vec<u32> = Self::parse_numbers(winning_numbers_parts[1]);
        let my_numbers: Vec<u32> = Self::parse_numbers(parts[1]);
        Scratchcard { winning_numbers, my_numbers }
    }

    fn parse_numbers(numbers_text: &str) -> Vec<u32> {
        numbers_text.split(' ')
            .filter(|item| item.len() > 0)
            .map(|item| item.parse::<u32>().unwrap())
            .collect()
    }

    pub fn worth(&self) -> u32 {
        let nr_matches = self.nr_winning_numbers();
        if nr_matches == 0 {
            0
        }
        else {
            1 << (nr_matches - 1)
        }
    }

    pub fn nr_winning_numbers(&self) -> u32 {
        let winning_set: HashSet<u32> = HashSet::from_iter(self.winning_numbers.clone());
        let my_set: HashSet<u32> = HashSet::from_iter(self.my_numbers.clone());
        winning_set.intersection(&my_set).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        // Arrange
        let line = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let expected = Scratchcard {
            winning_numbers: vec![1, 21, 53, 59, 44],
            my_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1]
        };

        // Act
        let actual = Scratchcard::parse(line);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_worth() {
        // Arrange
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratchcard = Scratchcard::parse(line);
        let expected: u32 = 8;

        // Act
        let actual: u32 = scratchcard.worth();

        // Assert
        assert_eq!(actual, expected);
    }
}