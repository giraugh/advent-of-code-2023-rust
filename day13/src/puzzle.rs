use std::{fmt::Display, iter};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, opt, value},
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Ash,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Row,
    Column,
}

pub type ReflectionLine = (Dir, usize);

#[derive(Debug, Clone)]
pub struct AshGrid {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl AshGrid {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, grid) = separated_list1(
            newline,
            many1(alt((
                value(Cell::Ash, tag(".")),
                value(Cell::Rock, tag("#")),
            ))),
        )(input)?;

        let height = grid.len();
        let width = grid[0].len();
        Ok((
            input,
            Self {
                grid,
                width,
                height,
            },
        ))
    }

    fn span(&self, dir: Dir, i: usize) -> Vec<Cell> {
        match dir {
            Dir::Row => (0..self.width).map(|x| self.grid[i][x]).collect(),
            Dir::Column => (0..self.height).map(|y| self.grid[y][i]).collect(),
        }
    }

    fn span_count(&self, dir: Dir) -> usize {
        match dir {
            Dir::Row => self.height,
            Dir::Column => self.width,
        }
    }

    fn opposing_spans(
        &self,
        dir: Dir,
        i: usize,
    ) -> impl Iterator<Item = (Vec<Cell>, Vec<Cell>)> + '_ {
        iter::zip((0..i).rev(), i..self.span_count(dir))
            .map(move |(ia, ib)| (self.span(dir, ia), self.span(dir, ib)))
    }

    /// Find a line which reflects all other columns
    fn scan_for_reflection(&self, dir: Dir) -> Option<usize> {
        (1..self.span_count(dir)).find(|&i| self.opposing_spans(dir, i).all(|(a, b)| a == b))
    }

    /// Find a line which *almost* reflects all other columns (1 char off)
    fn scan_for_alt_reflection(&self, dir: Dir) -> Option<usize> {
        (1..self.span_count(dir)).find(|&i| {
            self.opposing_spans(dir, i)
                .map(|(a, b)| iter::zip(a, b).filter(|(x, y)| x != y).count())
                .sum::<usize>()
                == 1
        })
    }

    /// The line of reflection (ceiled i.e if between 4 and 5 will return 5)
    fn line_of_reflection(&self) -> ReflectionLine {
        [Dir::Row, Dir::Column]
            .into_iter()
            .find_map(|dir| self.scan_for_reflection(dir).map(|i| (dir, i)))
            .unwrap()
    }

    /// The alternate line of reflection found by removing a smudge
    fn alt_line_of_reflection(&self) -> ReflectionLine {
        [Dir::Row, Dir::Column]
            .into_iter()
            .find_map(|dir| self.scan_for_alt_reflection(dir).map(|i| (dir, i)))
            .unwrap()
    }
}

type PuzzleInput = Vec<AshGrid>;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    all_consuming(terminated(
        separated_list1(tuple((newline, newline)), AshGrid::parse),
        opt(newline),
    ))(input_text)
    .unwrap()
    .1
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> usize {
    input
        .into_iter()
        .map(|grid| match grid.line_of_reflection() {
            (Dir::Row, row) => row * 100,
            (Dir::Column, col) => col,
        })
        .sum()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> usize {
    input
        .into_iter()
        .map(|grid| match grid.alt_line_of_reflection() {
            (Dir::Row, row) => row * 100,
            (Dir::Column, col) => col,
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt1(input)), "405");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "400");
    }
}
