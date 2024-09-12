pub fn get_regular_card_value(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Unknown card {}", card)
    }
}

pub fn get_joker_card_value(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => panic!("Unknown card {}", card)
    }
}

pub fn str_to_cards(s: &str) -> [char; 5] {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() != 5 {
        panic!("Input string must be exactly 5 characters long");
    }
    [chars[0], chars[1], chars[2], chars[3], chars[4]]
}
