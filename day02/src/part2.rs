use std::cmp;
use crate::helpers::{Game, Set};

pub fn process(input: &str) -> u32 {
    let output: u32 = input
        .lines()
        .filter_map(process_line)
        .sum();

    output
}

fn process_line(line: &str) -> Option<u32> {
    let game = Game::from(line);
    let _id = &game.id;
    let initial = Set {
        red: None,
        blue: None,
        green: None,
    };
    let max_set = game.sets
        .iter()
        .fold(initial, |acc, e|
            Set {
                red: cmp::max(acc.red, e.red),
                blue: cmp::max(acc.blue, e.blue),
                green: cmp::max(acc.green, e.green),
            },
        );

    let product = max_set.red.expect("should be a number") * max_set.blue.expect("should be a number") * max_set.green.expect("should be a number");
    Some(product)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process(input);
        assert_eq!(result, 2286);
    }
}