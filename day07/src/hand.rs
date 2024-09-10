use std::cmp::Ordering;
use std::collections::HashMap;
use crate::card::get_card_value;
use crate::char_frequency::CharFrequency;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [char; 5],
    hand_type: HandType
}

impl Hand {
    pub fn new(cards: &str) -> Hand {
        assert_eq!(cards.len(), 5);
        let cards = Self::str_to_cards(cards);
        Hand {
            cards,
            hand_type: Self::get_type_from_cards(cards)
        }
    }

    pub fn get_type(&self) -> &HandType {
        &self.hand_type
    }

    fn str_to_cards(s: &str) -> [char; 5] {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 5 {
            panic!("Input string must be exactly 5 characters long");
        }
        [chars[0], chars[1], chars[2], chars[3], chars[4]]
    }

    fn get_type_from_cards(cards: [char; 5]) -> HandType {
        let mut set: HashMap<char, u32> = HashMap::new();
        for ch in cards {
            if set.contains_key(&ch) {
                set.insert(ch, 1 + set.get(&ch).unwrap());
            }
            else {
                set.insert(ch, 1);
            }
        }

        let mut frequencies: Vec<CharFrequency> = set.iter()
            .map(|char_freq| CharFrequency::new(*char_freq.0, *char_freq.1))
            .collect();
        frequencies.sort();

        let top1_count = frequencies[0].get_frequency();
        if top1_count == 5 {
            return HandType::FiveOfAKind;
        }
        if top1_count == 4 {
            return HandType::FourOfAKind;
        }

        let top2_count = frequencies[1].get_frequency();
        if top1_count == 3 {
            return match top2_count {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind
            }
        }
        if top1_count == 2 {
            return match top2_count {
                2 => HandType::TwoPair,
                _ => HandType::OnePair
            }
        }

        HandType::HighCard
    }
}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cards == other.cards {
            return Ordering::Equal;
        }

        let type_ordering = self.get_type().cmp(other.get_type());
        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        for i in 0..self.cards.len() {
            let value_ordering = get_card_value(self.cards[i]).cmp(&get_card_value(other.cards[i]));
            if value_ordering != Ordering::Equal {
                return value_ordering;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_type_five_of_a_kind() {
        run_get_type_test_case("88888", HandType::FiveOfAKind);
    }

    #[test]
    fn test_get_type_four_of_a_kind() {
        run_get_type_test_case("88A88", HandType::FourOfAKind);
    }

    #[test]
    fn test_get_type_full_house() {
        run_get_type_test_case("88A8A", HandType::FullHouse);
    }

    #[test]
    fn test_get_type_three_of_a_kind() {
        run_get_type_test_case("88A83", HandType::ThreeOfAKind);
    }

    #[test]
    fn test_get_type_two_pair() {
        run_get_type_test_case("88A3A", HandType::TwoPair);
    }

    #[test]
    fn test_get_type_one_pair() {
        run_get_type_test_case("A8328", HandType::OnePair);
    }

    #[test]
    fn test_get_type_high_card() {
        run_get_type_test_case("T8A23", HandType::HighCard);
    }

    fn run_get_type_test_case(hand_text: &str, expected: HandType) {
        // Arrange
        let hand = Hand::new(hand_text);

        // Act
        let actual = hand.get_type();

        // Assert
        assert_eq!(*actual, expected);
    }

    #[test]
    fn test_cmp() {
        // Arrange
        let hand1 = Hand::new("KK677");
        let hand2 = Hand::new("KTJJT");
        let expected = Ordering::Greater;

        // Act
        let actual = hand1.cmp(&hand2);

        // Assert
        assert_eq!(actual, expected);
    }
}