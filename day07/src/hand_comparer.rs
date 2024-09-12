use crate::card::{get_joker_card_value, get_regular_card_value};
use crate::hand::Hand;
use std::cmp::Ordering;

pub trait HandComparer {
    fn compare_hands(&self, lhs: &Hand, rhs: &Hand) -> Ordering;
}

pub struct RegularHandComparer {}

impl HandComparer for RegularHandComparer {
    fn compare_hands(&self, lhs: &Hand, rhs: &Hand) -> Ordering {
        if lhs.get_cards() == rhs.get_cards() {
            return Ordering::Equal;
        }

        let type_ordering = lhs.get_type().cmp(rhs.get_type());
        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        for i in 0..lhs.get_cards().len() {
            let value_ordering = get_regular_card_value(lhs.get_cards()[i])
                .cmp(&get_regular_card_value(rhs.get_cards()[i]));
            if value_ordering != Ordering::Equal {
                return value_ordering;
            }
        }

        Ordering::Equal
    }
}

pub struct JokerHandComparer {}

impl HandComparer for JokerHandComparer {
    fn compare_hands(&self, lhs: &Hand, rhs: &Hand) -> Ordering {
        if lhs.get_cards() == rhs.get_cards() {
            return Ordering::Equal;
        }

        let type_ordering = lhs.get_type().cmp(rhs.get_type());
        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        for i in 0..lhs.get_cards().len() {
            let value_ordering = get_joker_card_value(lhs.get_cards()[i])
                .cmp(&get_joker_card_value(rhs.get_cards()[i]));
            if value_ordering != Ordering::Equal {
                return value_ordering;
            }
        }

        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand_categorizer::{JokerHandCategorizer, RegularHandCategorizer};

    #[test]
    fn test_regular_compare_two_pair() {
        run_regular_compare_test_case("KK677", "KTJJT", Ordering::Greater);
    }

    #[test]
    fn test_regular_compare_four_of_a_kind() {
        run_regular_compare_test_case("33332", "2AAAA", Ordering::Greater);
    }

    #[test]
    fn test_regular_compare_full_house() {
        run_regular_compare_test_case("77888", "77788", Ordering::Greater);
    }

    fn run_regular_compare_test_case(hand1: &str, hand2: &str, expected: Ordering) {
        // Arrange
        let comparer = RegularHandComparer {};

        // Act
        let actual = comparer.compare_hands(&str_to_hand(hand1), &str_to_hand(hand2));

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_joker_compare() {
        run_joker_compare_test_case("JJAA2", "AAA22", Ordering::Greater);
    }

    fn run_joker_compare_test_case(hand1: &str, hand2: &str, expected: Ordering) {
        // Arrange
        let comparer = JokerHandComparer {};

        // Act
        let actual = comparer.compare_hands(&str_to_joker_hand(hand1),
                                            &str_to_joker_hand(hand2));

        // Assert
        assert_eq!(actual, expected);
    }

    fn str_to_hand(s: &str) -> Hand {
        Hand::new(s, &RegularHandComparer {}, &RegularHandCategorizer {})
    }

    fn str_to_joker_hand(s: &str) -> Hand {
        Hand::new(s, &JokerHandComparer {}, &JokerHandCategorizer {})
    }
}