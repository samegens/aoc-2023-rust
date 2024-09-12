use std::cmp::Ordering;
use crate::card::get_card_value;
use crate::hand::Hand;

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
            let value_ordering = get_card_value(lhs.get_cards()[i])
                .cmp(&get_card_value(rhs.get_cards()[i]));
            if value_ordering != Ordering::Equal {
                return value_ordering;
            }
        }

        Ordering::Equal
    }
}
