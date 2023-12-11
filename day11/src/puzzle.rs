use std::fmt::Debug;

use derive_more::{Add, From, Into, Sub};
use itertools::Itertools;

type PuzzleInput = CosmicImage;

#[derive(Clone, Copy, PartialEq, Eq, Add, Sub, Into, From, Hash)]
struct Pos(isize, isize);

macro_rules! pos {
    ($x: expr, $y: expr) => {
        Pos($x as isize, $y as isize)
    };
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos({}, {})", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Galaxy,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CosmicImage {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
}

impl CosmicImage {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(From::from).collect_vec())
            .collect_vec();
        let height = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            height,
            width,
        }
    }

    /// Positions of galaxies in the image
    fn galaxy_positions(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..self.width)
            .cartesian_product(0..self.height)
            .filter(|&(x, y)| self.grid[y][x] == Cell::Galaxy)
            .map(|(x, y)| pos!(x, y))
    }

    /// Whether this row of the grid is actually twice as tall
    fn expanded_row(&self, y: usize) -> bool {
        (0..self.width).all(|x| self.grid[y][x] == Cell::Empty)
    }

    /// Whether this col of the grid is actually twice as wide
    fn expanded_col(&self, x: usize) -> bool {
        (0..self.height).all(|y| self.grid[y][x] == Cell::Empty)
    }

    /// Get the distance between two grid positions taking gravitational "stuff" into account
    fn cosmic_distance(&self, from: Pos, to: Pos, expansion: usize) -> usize {
        let mut pos = from;
        let mut distance = 0;
        while pos != to {
            if pos.0 != to.0 {
                pos.0 += (to.0 - pos.0).signum();
                distance += if self.expanded_col(pos.0 as usize) {
                    expansion
                } else {
                    1
                };
            } else {
                pos.1 += (to.1 - pos.1).signum();
                distance += if self.expanded_row(pos.1 as usize) {
                    expansion
                } else {
                    1
                };
            }
        }

        distance
    }
}

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    CosmicImage::parse(input_text)
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .galaxy_positions()
        .combinations(2)
        .filter_map(|c| (c[0] != c[1]).then(|| (c[0], c[1])))
        .map(|(a, b)| input.cosmic_distance(a, b, 2))
        .sum::<usize>()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .galaxy_positions()
        .combinations(2)
        .filter_map(|c| (c[0] != c[1]).then(|| (c[0], c[1])))
        .map(|(a, b)| input.cosmic_distance(a, b, 1000000))
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt1(input)), "374");
    }
}
