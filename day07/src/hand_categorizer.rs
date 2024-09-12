use crate::char_frequency::CharFrequency;
use crate::hand::HandType;
use std::collections::HashMap;

pub trait HandCategorizer {
    fn categorize(&self, cards: [char; 5]) -> HandType;
}

pub struct RegularHandCategorizer {}

impl HandCategorizer for RegularHandCategorizer {
    fn categorize(&self, cards: [char; 5]) -> HandType {
        let frequencies = get_card_frequencies(cards);

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

pub struct JokerHandCategorizer {}

impl HandCategorizer for JokerHandCategorizer {
    fn categorize(&self, cards: [char; 5]) -> HandType {
        let frequencies: Vec<CharFrequency> = get_card_frequencies(cards);
        let joker_count = frequencies
            .iter()
            .find(|char_freq| char_freq.get_char() == 'J')
            .map(|char_freq| char_freq.get_frequency())
            .unwrap_or(0);
        let frequencies: Vec<&CharFrequency> = frequencies.iter()
            .filter(|char_freq| char_freq.get_char() != 'J')
            .collect();

        let top1_count = frequencies.get(0)
            .map(|char_freq| char_freq.get_frequency())
            .unwrap_or(0);

        if top1_count + joker_count == 5 {
            return HandType::FiveOfAKind;
        }
        if top1_count + joker_count == 4 {
            return HandType::FourOfAKind;
        }

        let top2_count = frequencies.get(1)
            .map(|char_freq| char_freq.get_frequency())
            .unwrap_or(0);

        if top1_count + joker_count == 3 {
            return match top2_count {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind
            }
        }
        if top1_count + joker_count == 2 {
            return match top2_count {
                2 => HandType::TwoPair,
                _ => HandType::OnePair
            }
        }

        HandType::HighCard
    }
}

fn get_card_frequencies(cards: [char; 5]) -> Vec<CharFrequency> {
    let mut set: HashMap<char, u32> = HashMap::new();
    for ch in cards {
        if set.contains_key(&ch) {
            set.insert(ch, 1 + set.get(&ch).unwrap());
        } else {
            set.insert(ch, 1);
        }
    }

    let mut frequencies: Vec<CharFrequency> = set.iter()
        .map(|char_freq| CharFrequency::new(*char_freq.0, *char_freq.1))
        .collect();
    frequencies.sort();
    frequencies
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::str_to_cards;

    #[test]
    fn test_get_type_five_of_a_kind() {
        run_regular_categorize_test_case("88888", HandType::FiveOfAKind);
    }

    #[test]
    fn test_get_type_four_of_a_kind() {
        run_regular_categorize_test_case("88A88", HandType::FourOfAKind);
    }

    #[test]
    fn test_get_type_full_house() {
        run_regular_categorize_test_case("88A8A", HandType::FullHouse);
    }

    #[test]
    fn test_get_type_three_of_a_kind() {
        run_regular_categorize_test_case("88A83", HandType::ThreeOfAKind);
    }

    #[test]
    fn test_get_type_two_pair() {
        run_regular_categorize_test_case("88A3A", HandType::TwoPair);
    }

    #[test]
    fn test_get_type_one_pair() {
        run_regular_categorize_test_case("A8328", HandType::OnePair);
    }

    #[test]
    fn test_get_type_high_card() {
        run_regular_categorize_test_case("T8A23", HandType::HighCard);
    }

    fn run_regular_categorize_test_case(hand_text: &str, expected: HandType) {
        // Arrange
        let categorizer = RegularHandCategorizer {};

        // Act
        let actual = categorizer.categorize(str_to_cards(hand_text));

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_type_from_joker_cards_five_of_a_kind() {
        run_joker_categorize_test_case("KJKJK", HandType::FiveOfAKind);
    }

    #[test]
    fn test_get_type_from_joker_cards_four_of_a_kind() {
        run_joker_categorize_test_case("KJQJK", HandType::FourOfAKind);
    }

    #[test]
    fn test_get_type_from_joker_cards_four_of_a_kind_many_jokers() {
        run_joker_categorize_test_case("KJJJQ", HandType::FourOfAKind);
    }

    #[test]
    fn test_get_type_from_joker_cards_full_house() {
        run_joker_categorize_test_case("KKQJQ", HandType::FullHouse);
    }

    #[test]
    fn test_get_type_from_joker_cards_three_of_a_kind() {
        run_joker_categorize_test_case("KJQJA", HandType::ThreeOfAKind);
    }

    #[test]
    fn test_get_type_from_joker_cards_two_pair() {
        // We can't get two pair using one or more jokers.
        run_joker_categorize_test_case("K2K2A", HandType::TwoPair);
    }

    #[test]
    fn test_get_type_from_joker_cards_one_pair() {
        run_joker_categorize_test_case("KJQ2A", HandType::OnePair);
    }

    fn run_joker_categorize_test_case(hand_text: &str, expected: HandType) {
        // Arrange
        let categorizer = JokerHandCategorizer {};

        // Act
        let actual = categorizer.categorize(str_to_cards(hand_text));

        // Assert
        assert_eq!(actual, expected);
    }
}
