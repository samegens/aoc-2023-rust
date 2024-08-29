use crate::game_set::GameSet;

#[derive(Debug, PartialEq)]
pub struct GameId(u32);

impl GameId {
    pub fn to_u32(&self) -> u32 {
        self.0
    }
}
#[derive(Debug, PartialEq)]
pub struct Game {
    id: GameId,
    game_sets: Vec<GameSet>,
}

impl Game {
    pub fn new(id: GameId) -> Self {
        Game {
            id,
            game_sets: Vec::new()
        }
    }

    pub fn id(&self) -> &GameId {
        &self.id
    }

    pub(crate) fn add(&mut self, game_set: GameSet) {
        self.game_sets.push(game_set);
    }

    /// Parse a line of the form
    /// 'Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red'
    pub fn parse(game_text: &str) -> Self {
        let parts: Vec<&str> = game_text.split(':').collect();
        let game_id_parts: Vec<&str> = parts[0].split_whitespace().collect();
        let id: GameId = GameId(game_id_parts[1].parse::<u32>().unwrap());
        // let mut game_sets: Vec<GameSet> = Vec::new();

        let game_sets: Vec<GameSet> = parts[1]
            .split(';')
            .map(|game_set_text|GameSet::parse(game_set_text))
            .collect();

        Game {
            id,
            game_sets
        }
    }

    pub fn is_possible_part1(&self) -> bool {
        for game_set in self.game_sets.iter() {
            if !game_set.is_possible_part1() {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        // Arrange
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let mut expected: Game = Game::new(GameId(3));
        expected.add(GameSet::parse("8 green, 6 blue, 20 red"));
        expected.add(GameSet::parse("5 blue, 4 red, 13 green"));
        expected.add(GameSet::parse("5 green, 1 red"));

        // Act
        let actual = Game::parse(input);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_possible_with_too_many_red() {
        run_is_possible_test_case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", false);
    }

    #[test]
    fn test_is_possible_with_possible_game() {
        run_is_possible_test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true);
    }

    fn run_is_possible_test_case(game_text: &str, expected: bool) {
        // Arrange
        let game: Game = Game::parse(game_text);

        // Act
        let actual: bool = game.is_possible_part1();

        // Assert
        assert_eq!(actual, expected);
    }
}
