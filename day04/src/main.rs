use std::str::Lines;
use common::InputReader;
use scratchcard::Scratchcard;

mod scratchcard;

fn main() {
    let input_reader: InputReader = InputReader::new(4);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> u32 {
    lines
        .map(|line| Scratchcard::parse(line).worth())
        .sum()
}

fn solve_part2(lines: Lines) -> u32 {
    let nr_winning_numbers_per_card: Vec<u32> = lines
        .map(|line| Scratchcard::parse(line).nr_winning_numbers())
        .collect();
    let mut nr_cards: Vec<u32> = vec![1; nr_winning_numbers_per_card.len()];
    for i in 0..nr_winning_numbers_per_card.len() {
        for j in i + 1..=i + nr_winning_numbers_per_card[i] as usize {
            if j < nr_cards.len() {
                nr_cards[j] = nr_cards[j] + nr_cards[i];
            }
        }
    }

    nr_cards.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_solve_part1()
    {
        // Arrange
        let expected: u32 = 13;

        // Act
        let actual: u32 = solve_part1(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2()
    {
        // Arrange
        let expected: u32 = 30;

        // Act
        let actual: u32 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }
}
