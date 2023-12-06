use nom::{
    bytes::complete::{tag, take_till1},
    character::complete,
    character::complete::{digit1, space1},
    IResult,
    multi::separated_list1,
    sequence::preceded,
};

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Card {
    pub id: usize,
    pub winning: Vec<u32>,
    pub draw: Vec<u32>,
}

impl Card {
    fn parse_set(input: &str) -> IResult<&str, Vec<u32>> {
        let (input, list) = separated_list1(space1, complete::u32)(input.trim())?;
        Ok((input, list))
    }

    pub fn calculate_matches(winning: &Vec<u32>, draw: &Vec<u32>) -> u32 {
        let mut matches: u32 = 0;
        for num in draw {
            let search_result = winning.iter().find(|x| *x == num);
            match search_result {
                Some(_) => { matches += 1 }
                _ => {}
            };
        }

        matches
    }
    pub fn calculate_points(winning: &Vec<u32>, draw: &Vec<u32>) -> u32 {
        let mut points: u32 = 0;
        for num in draw {
            let search_result = winning.iter().find(|x| *x == num);
            match search_result {
                Some(_) => {
                    if points == 0 {
                        points += 1;
                    } else {
                        points *= 2;
                    }
                }
                _ => {}
            };
        }

        points
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, id) = preceded(take_till1(|c: char| c.is_ascii_digit()), digit1)(input)?;
        let (input, sets) = preceded(
            tag(": "),
            separated_list1(
                tag(" | "),
                Card::parse_set,
            ),
        )(input)?;
        let winning = sets.first().expect("should be vec of u32").clone();
        let draw = sets.last().expect("should be vec of u32").clone();

        Ok((input, Card {
            id: id.parse::<usize>().expect("should be usize"),
            winning,
            draw,
        }))
    }
}