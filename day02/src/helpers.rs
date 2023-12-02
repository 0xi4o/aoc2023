use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Set {
    pub red: Option<u32>,
    pub blue: Option<u32>,
    pub green: Option<u32>,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

impl Set {
    pub fn from(input: &str) -> Self {
        let sets_vec = input.split(", ").collect::<Vec<&str>>();
        let mut sets_map: HashMap<&str, u32> = HashMap::new();
        for set in sets_vec {
            let (color, val) = Set::parse_set(set);
            sets_map.insert(color, val);
        }

        Set {
            red: sets_map.remove("red"),
            blue: sets_map.remove("blue"),
            green: sets_map.remove("green"),
        }
    }

    pub fn parse_set(input: &str) -> (&str, u32) {
        let set = input.split_whitespace().collect::<Vec<&str>>();
        let color = set.last().expect("should be a string").to_owned();
        let num = set.first().expect("should be a string").parse::<u32>().expect("should be a number");
        (color, num)
    }
}

impl Game {
    pub fn from(input: &str) -> Self {
        let game = input.split(": ").collect::<Vec<&str>>();
        let id = game
            .first()
            .expect("should be a string")
            .to_owned()
            .replace("Game ", "")
            .parse::<u32>()
            .expect("should be a number");
        let value = game.last().expect("should be a string").to_owned();
        let sets = value.split("; ");
        let sets = sets.map(Set::from).collect::<Vec<Set>>();

        Game {
            id,
            sets,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_from() {
        let input = "1 red, 2 green";
        let result = Set::from(input);
        let expected = Set {
            red: Some(1),
            blue: None,
            green: Some(2),
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_set_parse() {
        let input = "1 red";
        let result = Set::parse_set(input);
        assert_eq!(result, ("red", 1));
    }

    #[test]
    fn test_game_from() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = Game::from(input);
        let sets = vec![
            Set { red: Some(4), blue: Some(3), green: None },
            Set { red: Some(1), blue: Some(6), green: Some(2) },
            Set { red: None, blue: None, green: Some(2) },
        ];
        let expected = Game {
            id: 1,
            sets,
        };
        assert_eq!(result, expected);
    }
}