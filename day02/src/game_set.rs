use std::collections::HashMap;
use crate::cube_color::CubeColor;

#[derive(Debug, PartialEq)]
pub struct GameSet(HashMap<CubeColor, i32>);

impl GameSet {
    pub(crate) fn is_possible_part1(&self) -> bool {
        if self.0.contains_key(&CubeColor::Red) && self.0[&CubeColor::Red] > 12 {
            return false;
        }
        if self.0.contains_key(&CubeColor::Green) && self.0[&CubeColor::Green] > 13 {
            return false;
        }
        if self.0.contains_key(&CubeColor::Blue) && self.0[&CubeColor::Blue] > 14 {
            return false;
        }

        true
    }
}

impl GameSet {
    pub fn new() -> Self {
        GameSet(HashMap::new())
    }

    pub fn parse(game_set_text: &str) -> Self {
        let parts: Vec<&str> = game_set_text.split(',').collect();
        let mut map: HashMap<CubeColor, i32> = HashMap::new();
        for part in parts {
            let sub_parts: Vec<&str> = part.split_whitespace().collect();
            let color = sub_parts[1].parse::<CubeColor>().unwrap();
            let amount: i32 = sub_parts[0].parse::<i32>().unwrap();

            map.insert(color, amount);
        }

        GameSet(map)
    }

    pub fn insert(&mut self, key: CubeColor, amount: i32) {
        self.0.insert(key, amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        // Arrange
        let input = " 1 red, 2 green, 6 blue";
        let mut expected: GameSet = GameSet::new();
        expected.insert(CubeColor::Red, 1);
        expected.insert(CubeColor::Green, 2);
        expected.insert(CubeColor::Blue, 6);

        // Act
        let actual = GameSet::parse(input);

        // Assert
        assert_eq!(actual, expected);
    }
}
