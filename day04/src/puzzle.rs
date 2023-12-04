use nom::{
    bytes::complete::tag,
    character::complete::{u32, multispace1},
    combinator::{map, all_consuming},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult, branch::alt,
};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Card {
    winning: Vec<u32>,
    owned: Vec<u32>,
}

impl Card {
    fn winning_numbers(&self) -> usize {
        self.owned
            .iter()
            .filter(|num| self.winning.contains(num))
            .count()
    } 

    fn points(&self) -> u32 {
        let count = self.winning_numbers();
        if count > 0 {
            2u32.pow((count - 1) as u32)
        } else {
            0
        }
    }

    #[rustfmt::skip]
    fn parse(input: &str) -> IResult<&str, Self> {
        map(tuple((
            preceded(tuple((tag("Card"), multispace1)), u32),
            preceded(
                alt((tag(":  "), tag(": "))),
                separated_list1(multispace1, u32),
            ),
            preceded(
                alt((tag(" |  "), tag(" | "))),
                separated_list1(multispace1, u32)
            )
        )), |(_, winning, owned)| Self { 
                winning, owned
            })(input)
    }
}

type PuzzleInput = Vec<Card>;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    input_text.lines().map(|l| all_consuming(Card::parse)(l).unwrap().1).collect_vec()
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    input.iter().map(|card| card.points()).sum::<u32>()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    // We initially have a copy of every card
    let mut total_owned = input.len();
    let mut stack = Vec::from_iter(0..input.len());

    // Start checking our wins
    //   could prob do this more efficient by tracking the count of each card
    //   but it runs fast enough :)
    while let Some(card_index) = stack.pop() {
        let card = input.get(card_index).unwrap();
        let wins = card.winning_numbers();
        let (start, stop) = (card_index + 1, card_index + wins + 1);
        stack.extend(start..stop);
        total_owned += wins;
    }

    total_owned
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let cards = parse_input(SAMPLE_TEXT);
        assert_eq!(cards.len(), 6);
        assert_eq!(format!("{:?}", solve_pt1(cards)), "13");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let cards = parse_input(SAMPLE_TEXT);
        assert_eq!(cards.len(), 6);
        assert_eq!(format!("{:?}", solve_pt2(cards)), "30");
    }
}
