use std::collections::HashSet;

use aoc::{
    direction::{Dir, OrthDir},
    grid::{Grid, GridPos},
    pos,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirrorDir {
    /// `/`
    /// rotates horizontal light to its left
    Left,

    /// `\`
    /// rotates horizontal light to its right
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Splitter(OrthDir),
    Mirror(MirrorDir),
}

impl Cell {
    fn parse(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '-' => Cell::Splitter(OrthDir::Horizontal),
            '|' => Cell::Splitter(OrthDir::Vertical),
            '/' => Cell::Mirror(MirrorDir::Left),
            '\\' => Cell::Mirror(MirrorDir::Right),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Floor {
    layout: Grid<Cell>,
    dir_history: Grid<HashSet<Dir>>,
}

impl Floor {
    fn parse(input: &str) -> Self {
        let layout = input
            .lines()
            .map(|line| line.chars().map(Cell::parse).collect())
            .collect();

        let layout = Grid::new(layout);
        Self {
            dir_history: Grid::from_default(layout.width, layout.height),
            layout,
        }
    }

    fn trace_beam(&mut self, from: GridPos, direction: Dir) {
        // Find next non empty point
        let mut pos = from;
        while self.layout.get(pos) == Some(Cell::Empty) {
            if !self.dir_history.get_mut(pos).unwrap().insert(direction) {
                return;
            }

            pos += direction.into();
        }

        // Where did we end up?
        let cell = match self.layout.get(pos) {
            // Did we go outside the grid?
            None => {
                return;
            }

            // In the grid?
            Some(cell) => cell,
        };

        // Record this pos+dir
        // If seen before, exit early
        if !self.dir_history.get_mut(pos).unwrap().insert(direction) {
            return;
        }

        // Did we go outside the grid? if so terminate
        match cell {
            // Splitter that splits
            Cell::Splitter(orth_dir) if orth_dir != direction.orthogonal() => {
                self.trace_beam(pos + direction.turn_left().into(), direction.turn_left());
                self.trace_beam(pos + direction.turn_right().into(), direction.turn_right())
            }

            // Splitter that doesn't split
            Cell::Splitter(_) => self.trace_beam(pos + direction.into(), direction),

            Cell::Mirror(mirror_dir) => {
                let new_dir = match (mirror_dir, direction.orthogonal()) {
                    (MirrorDir::Left, OrthDir::Horizontal) => direction.turn_left(),
                    (MirrorDir::Left, OrthDir::Vertical) => direction.turn_right(),
                    (MirrorDir::Right, OrthDir::Horizontal) => direction.turn_right(),
                    (MirrorDir::Right, OrthDir::Vertical) => direction.turn_left(),
                };

                self.trace_beam(pos + new_dir.into(), new_dir)
            }

            Cell::Empty => unreachable!(),
        }
    }

    fn energy_level(&self) -> usize {
        self.dir_history
            .cells_iter()
            .filter(|c| !c.is_empty())
            .count()
    }
}

type PuzzleInput = Floor;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    Floor::parse(input_text)
}

/// Solve puzzle part 1
pub fn solve_pt1(mut input: PuzzleInput) -> impl std::fmt::Debug {
    input.trace_beam(pos!(0, 0), Dir::East);
    input.energy_level()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> usize {
    let (width, height) = (input.layout.width, input.layout.height);
    let left = (0..height).map(|y| (pos!(0, y), Dir::East));
    let right = (0..height).map(|y| (pos!(width - 1, y), Dir::West));
    let top = (0..width).map(|x| (pos!(x, 0), Dir::South));
    let bottom = (0..width).map(|x| (pos!(x, height - 1), Dir::North));

    left.chain(right)
        .chain(top)
        .chain(bottom)
        .map(|(from, direction)| {
            let mut input = input.clone();
            input.trace_beam(from, direction);
            input.energy_level()
        })
        .max()
        .expect("At least one input")
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt1(input)), "46");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "51");
    }
}
