
use nom::{
    bytes::complete::{tag},
    character::complete::{digit1, line_ending},
    combinator::{map, opt},
    multi::{ separated_list1},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct FileData {
    pub rules: Vec<(u32, u32)>,
    pub seqs: Vec<Vec<u32>>,
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map(digit1, |s: &str| s.parse::<u32>().unwrap())(input)
}

fn parse_rule(input: &str) -> IResult<&str, (u32, u32)> {
    map(
        separated_list1(tag("|"), parse_number),
        |nums: Vec<u32>| (nums[0], nums[1]),
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(line_ending, parse_rule)(input)
}

fn parse_seq(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), parse_number)(input)
}

fn parse_seqs(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, parse_seq)(input)
}

pub fn parse_file(input: &str) -> IResult<&str, FileData> {
    let (input, pipe_grid) = parse_rules(input)?;
    let (input, _) = line_ending(input)?; // Consume the newline between sections
    let (input, _) = opt(line_ending)(input)?; // Handle optional blank lines
    let (input, comma_grid) = parse_seqs(input)?;
    Ok((input, FileData { rules: pipe_grid, seqs: comma_grid }))
}