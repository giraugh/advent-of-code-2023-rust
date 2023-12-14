use derive_more::{Add, From, Into, Mul, Sub};
use itertools::{repeat_n, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

#[derive(Clone, Copy, PartialEq, Eq, Add, Sub, Mul, Into, From, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Round,
    Cube,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

const CYCLE_COUNT: usize = 1000000000;
const CYCLE_DIRECTIONS: [Dir; 4] = {
    use Dir::*;
    [North, West, South, East]
};

#[derive(Clone)]
pub struct RockGrid {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Debug for RockGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self.grid[y][x] {
                        Cell::Round => "O",
                        Cell::Cube => "#",
                        Cell::Empty => ".",
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl RockGrid {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, grid) = separated_list1(
            newline,
            many1(alt((
                value(Cell::Empty, tag(".")),
                value(Cell::Cube, tag("#")),
                value(Cell::Round, tag("O")),
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

    fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height).cartesian_product(0..self.width)
    }

    fn spin_cycle(&mut self) {
        for dir in CYCLE_DIRECTIONS {
            self.roll(dir)
        }
    }

    fn get_unchecked(&self, pos: Pos) -> Cell {
        self.grid[pos.1 as usize][pos.0 as usize]
    }

    fn get(&self, pos: Pos) -> Option<Cell> {
        if pos.0 < 0 || pos.0 >= self.width as isize || pos.1 < 0 || pos.1 >= self.height as isize {
            return None;
        }
        Some(self.grid[pos.1 as usize][pos.0 as usize])
    }

    fn roll(&mut self, dir: Dir) {
        // The order we look at cells depends on the direction
        // (I guess I could like double buffer or somethin but ehh)
        let (xr, yr) = (0..self.width, 0..self.height);
        let pos_iter: Box<dyn Iterator<Item = Pos>> = match dir {
            Dir::North => Box::new(yr.cartesian_product(xr).map(|(y, x)| pos!(x, y))),
            Dir::East => Box::new(xr.rev().cartesian_product(yr).map(|(x, y)| pos!(x, y))),
            Dir::South => Box::new(yr.rev().cartesian_product(xr).map(|(y, x)| pos!(x, y))),
            Dir::West => Box::new(xr.cartesian_product(yr).map(|(x, y)| pos!(x, y))),
        };
        let pos_delta = match dir {
            Dir::North => pos!(0, -1),
            Dir::South => pos!(0, 1),
            Dir::East => pos!(1, 0),
            Dir::West => pos!(-1, 0),
        };

        for pos in pos_iter {
            if self.get_unchecked(pos) == Cell::Round {
                let move_by = (1..)
                    .map(|amt| pos!(pos_delta.0 * amt, pos_delta.1 * amt))
                    .take_while(|&move_by| self.get(pos + move_by) == Some(Cell::Empty))
                    .last();
                if let Some(move_by) = move_by {
                    let new_pos = pos + move_by;
                    self.grid[pos.1 as usize][pos.0 as usize] = Cell::Empty;
                    self.grid[new_pos.1 as usize][new_pos.0 as usize] = Cell::Round;
                }
            }
        }
    }

    fn north_load(&self) -> usize {
        self.positions()
            .filter(|&(x, y)| self.grid[y][x] == Cell::Round)
            .map(|(_x, y)| self.height - y)
            .sum()
    }
}

type PuzzleInput = RockGrid;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    RockGrid::parse(input_text).unwrap().1
}

/// Solve puzzle part 1
pub fn solve_pt1(mut input: PuzzleInput) -> impl std::fmt::Debug {
    input.roll(Dir::North);
    input.north_load()
}

/// Solve puzzle part 2
pub fn solve_pt2(mut input: PuzzleInput) -> impl std::fmt::Debug {
    // Look for a cycle
    let mut history: HashMap<Vec<Vec<Cell>>, usize> = HashMap::new();
    let mut left_over_cycles: usize = 0;
    for cycle in 0..CYCLE_COUNT {
        input.spin_cycle();
        if let Some(historic_cycle) = history.get(&input.grid) {
            // If we repeat this cycle, how many would be left?
            let remaining = CYCLE_COUNT - cycle;
            let cycle_length = cycle - historic_cycle;
            left_over_cycles = (remaining % cycle_length) - 1;
            break;
        } else {
            history.insert(input.grid.clone(), cycle);
        }
    }

    // Do the final few
    for _ in 0..left_over_cycles {
        input.spin_cycle();
    }

    input.north_load()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt1(input)), "136");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "64");
    }
}
