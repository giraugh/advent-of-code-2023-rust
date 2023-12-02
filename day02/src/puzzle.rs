use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::{all_consuming, map, opt},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, derive_more::Sum, derive_more::Add)]
pub struct Rgb(u32, u32, u32);

impl Rgb {
    pub fn power(&self) -> u32 {
        self.0 * self.1 * self.2
    }
}

impl PartialOrd for Rgb {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else if self.0 > other.0 || self.1 > other.1 || self.2 > other.2 {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

pub type PuzzleInput = Vec<Vec<Rgb>>;

/// Parse a count of red, green and blue cubes
#[rustfmt::skip]
fn parse_rgb(input: &str) -> IResult<&str, Rgb> {
    // Parse each item of set
    let (s, items) = many1(terminated(
        alt((
            map(terminated(u32, tag(" red")),   |x| Rgb(x, 0, 0)),
            map(terminated(u32, tag(" green")), |x| Rgb(0, x, 0)),
            map(terminated(u32, tag(" blue")),  |x| Rgb(0, 0, x)),
        )), 
        opt(tag(", "))
    ))(input)?;

    // Reduce them
    let rgb = items.into_iter().sum();
    Ok((s, rgb))
}

/// Parse a game log line
#[rustfmt::skip]
fn parse_game(input: &str) -> IResult<&str, Vec<Rgb>> {
    all_consuming(preceded(tag("Game "), preceded(
        u32,
        preceded(tag(": "), separated_list1(tag("; "), parse_rgb))
    )))(input)
}

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    input_text
        .lines()
        .map(|l| parse_game(l).expect("Valid game").1)
        .collect()
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    const AVAILABLE_RGB: Rgb = Rgb(12, 13, 14);
    input
        .into_iter()
        .enumerate()
        .filter(|(_, sets)| !sets.iter().any(|rgb| *rgb > AVAILABLE_RGB))
        .map(|(i, _)| i + 1)
        .sum::<usize>()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .into_iter()
        .map(|sets| {
            let r = *sets.iter().map(|Rgb(r, _, _)| r).max().unwrap();
            let g = *sets.iter().map(|Rgb(_, g, _)| g).max().unwrap();
            let b = *sets.iter().map(|Rgb(_, _, b)| b).max().unwrap();
            Rgb(r, g, b)
        })
        .map(|rgb| rgb.power())
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert!(input.is_ok());
        assert_eq!(input.unwrap().1.len(), 3);
    }

    #[test]
    fn tuple_comp_test() {
        assert!(Rgb(1, 2, 0) < Rgb(3, 4, 0));
        assert!(Rgb(1, 5, 0) > Rgb(2, 2, 0));
    }

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(include_str!("../sample.txt"));
        assert_eq!(format!("{:?}", solve_pt1(input)), "8");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(include_str!("../sample.txt"));
        assert_eq!(format!("{:?}", solve_pt2(input)), "2286");
    }
}
