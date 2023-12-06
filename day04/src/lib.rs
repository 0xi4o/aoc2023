use std::collections::HashSet;
use nom::{
    bytes::complete::{tag, take_till1},
    character::complete,
    character::complete::{digit1, space0},
    IResult,
    multi::fold_many1,
    sequence::{preceded, separated_pair, terminated},
};

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Card {
    pub winning: HashSet<u32>,
    pub draw: HashSet<u32>,
}

impl Card {
    fn parse_set(input: &str) -> IResult<&str, HashSet<u32>> {
        fold_many1(
            terminated(complete::u32, space0),
            HashSet::new,
            |mut acc, item| {
                acc.insert(item);
                acc
            },
        )(input.trim())
    }

    pub fn calculate_matches(&self) -> u32 {
        self.winning
            .intersection(&self.draw)
            .count() as u32
    }
    pub fn calculate_points(&self) -> u32 {
        match self.calculate_matches().checked_sub(1) {
            Some(num) => 2u32.pow(num),
            None => 0
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = preceded(take_till1(|c: char| c.is_ascii_digit()), digit1)(input)?;
        let (input, (winning, draw)) = preceded(
            tag(": "),
            separated_pair(
                Card::parse_set,
                tag("|"),
                Card::parse_set,
            ),
        )(input)?;

        Ok((input, Card {
            winning,
            draw,
        }))
    }
}