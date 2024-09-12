use crate::hand_categorizer::HandCategorizer;
use crate::hand_comparer::HandComparer;
use std::cmp::Ordering;
use std::fmt;
use crate::card::str_to_cards;

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

// Since Hand.comparer should not be part of default implementation of Debug, PartialEq and Eq,
// we can't derive Hand anymore, so we implement them ourselves:
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
    pub fn new(cards: &str, comparer: &'a dyn HandComparer, categorizer: &dyn HandCategorizer) -> Hand<'a> {
        assert_eq!(cards.len(), 5);
        let cards = str_to_cards(cards);
        Hand {
            cards,
            hand_type: categorizer.categorize(cards),
            comparer,
        }
    }

    pub fn get_type(&self) -> &HandType {
        &self.hand_type
    }

    pub fn get_cards(&self) -> &[char] {
        &self.cards
    }
}

// We implement Ord on Hand so we can simply sort a Vec<Hand>.
impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Delegate the actual comparing to the injected comparer.
        self.comparer.compare_hands(self, other)
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
