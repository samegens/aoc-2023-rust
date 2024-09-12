use crate::char_frequency::CharFrequency;
use crate::hand_comparer::HandComparer;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

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

pub struct Hand<'a> {
    cards: [char; 5],
    hand_type: HandType,
    comparer: &'a dyn HandComparer
}

impl<'a> fmt::Debug for Hand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Hand")
            .field("cards", &self.cards)
            .field("hand_type", &self.hand_type)
            .finish() // Comparer is ignored
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type
    }
}

// Manual implementation of Eq (no extra methods needed since PartialEq already does the work)
impl<'a> Eq for Hand<'a> {}

impl<'a> Hand<'a> {
    pub fn new(cards: &str, comparer: &'a dyn HandComparer) -> Hand<'a> {
        assert_eq!(cards.len(), 5);
        let cards = Self::str_to_cards(cards);
        Hand {
            cards,
            hand_type: Self::get_type_from_cards(cards),
            comparer,
        }
    }

    pub fn get_type(&self) -> &HandType {
        &self.hand_type
    }

    pub fn get_cards(&self) -> &[char] {
        &self.cards
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

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.comparer.compare_hands(self, other)
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::hand_comparer::RegularHandComparer;
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
        let hand = Hand::new(hand_text, &RegularHandComparer {});

        // Act
        let actual = hand.get_type();

        // Assert
        assert_eq!(*actual, expected);
    }

    #[test]
    fn test_cmp() {
        // Arrange
        let hand1 = Hand::new("KK677", &RegularHandComparer {});
        let hand2 = Hand::new("KTJJT", &RegularHandComparer {});
        let expected = Ordering::Greater;

        // Act
        let actual = hand1.cmp(&hand2);

        // Assert
        assert_eq!(actual, expected);
    }
}