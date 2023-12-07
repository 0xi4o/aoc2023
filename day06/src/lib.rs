use std::iter::zip;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{digit1, line_ending, space0, space1};
use nom::IResult;
use nom::multi::{fold_many1, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated};

pub mod part1;
pub mod part2;


fn parse_list(input: &str) -> IResult<&str, Vec<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        Vec::new,
        |mut acc, item| {
            acc.push(item);
            acc
        },
    )(input.trim())
}

fn parse_str(input: &str) -> IResult<&str, u64> {
    let (input, x) = separated_list1(space1, digit1)(input.trim())?;
    let val = x.join("").parse::<u64>().expect("should be a u64");
    Ok((input, val))
}

fn parse_race(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, race) = separated_pair(
        preceded(tag("Time:"), parse_str),
        line_ending,
        preceded(tag("Distance:"), parse_str),
    )(input)?;
    Ok((input, race))
}

pub fn parse_races(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, (times, distances)) = separated_pair(
        preceded(tag("Time:"), parse_list),
        line_ending,
        preceded(tag("Distance:"), parse_list),
    )(input)?;
    let races: Vec<_> = zip(times, distances).collect();
    Ok((input, races))
}