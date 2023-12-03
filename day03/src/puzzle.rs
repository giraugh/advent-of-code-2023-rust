use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Hash, derive_more::From)]
pub struct GridPos(pub usize, pub usize);

#[derive(Debug, Clone)]
pub struct CharGrid {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl FromStr for CharGrid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The length of a line is the width
        // the amount of lines is the height
        let mut lines = s.lines().peekable();
        let width = lines.peek().unwrap().len();
        let height = lines.count();
        Ok(Self {
            data: s.chars().filter(|c| *c != '\n').collect(),
            width,
            height,
        })
    }
}

impl GridPos {
    pub fn neighbours(&self) -> impl Iterator<Item = GridPos> {
        let (x, y) = (self.0 as isize, self.1 as isize);
        [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
        ]
        .into_iter()
        .filter(|&(x, y)| x >= 0 && y >= 0)
        .map(|(x, y)| GridPos(x as usize, y as usize))
    }
}

impl CharGrid {
    pub fn inbounds(&self, pos: &GridPos) -> bool {
        (0..self.width).contains(&pos.0) && (0..self.height).contains(&pos.1)
    }

    pub fn index_to_pos(&self, index: usize) -> GridPos {
        let y = index.div_floor(self.width);
        let x = index % self.width;
        (x, y).into()
    }

    pub fn pos_to_index(&self, pos: GridPos) -> usize {
        debug_assert!(pos.0 < self.width && pos.1 < self.height);
        pos.1 * self.width + pos.0
    }

    pub fn at(&self, pos: GridPos) -> Option<&char> {
        self.data.get(self.pos_to_index(pos))
    }

    pub fn symbol_locations(&self) -> impl Iterator<Item = GridPos> + '_ {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, &c)| !c.is_alphanumeric() && c != '.')
            .map(|(i, _)| self.index_to_pos(i))
    }

    pub fn part_numbers(&self) -> Vec<(Range<usize>, usize)> {
        let mut part_numbers = Vec::new();

        // First find spaces near symbols
        let symbol_surrounds: HashSet<_> = self
            .symbol_locations()
            .flat_map(|pos| pos.neighbours())
            .filter(|pos| self.inbounds(pos))
            .map(|pos| self.pos_to_index(pos))
            .collect();

        // Then look for numbers
        let mut buffer = Vec::new();
        let mut near_symbol = false;
        let mut buffer_start = 0;

        for (i, c) in self.data.iter().enumerate() {
            if c.is_ascii_digit() {
                if buffer.is_empty() {
                    buffer_start = i;
                }
                buffer.push(c);
                if symbol_surrounds.contains(&i) {
                    near_symbol = true;
                }
            }

            if !c.is_ascii_digit() || (i + 1) % self.width == 0 {
                // If it had a surround, store it
                if !buffer.is_empty() {
                    let buffer_chars = buffer.drain(0..);
                    if near_symbol {
                        let part_number = buffer_chars.collect::<String>();
                        let part_number = part_number.parse::<usize>().unwrap();
                        part_numbers.push((buffer_start..i, part_number));
                    }
                }

                // Reset flag
                near_symbol = false;
            }
        }

        part_numbers
    }
}

type PuzzleInput = CharGrid;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    input_text.parse().unwrap()
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .part_numbers()
        .iter()
        .map(|(_, num)| num)
        .sum::<usize>()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    // First construct a map from character index to gear id and number
    let gear_indices: HashMap<usize, (usize, usize)> = input
        .part_numbers()
        .into_iter()
        .enumerate()
        .flat_map(|(id, (range, value))| range.into_iter().map(move |i| (i, (id, value))))
        .collect();

    // Look for "gears"
    input
        .data
        .iter()
        .enumerate()
        .filter(|&(_, c)| *c == '*')
        .map(|(i, _)| input.index_to_pos(i))
        .filter_map(|pos| {
            let gear_parts = pos
                .neighbours()
                .map(|np| input.pos_to_index(np))
                .flat_map(|ni| gear_indices.get(&ni))
                .unique_by(|(gear_id, _)| gear_id)
                .map(|(_, gear_value)| gear_value)
                .collect_vec();
            if gear_parts.len() == 2 {
                Some(gear_parts.into_iter().product::<usize>())
            } else {
                None
            }
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");
    const SAMPLE_2: &str = include_str!("../sample2.txt");

    /// Test parse puzzle input
    #[test]
    fn parsing_input() {
        let grid = parse_input(SAMPLE);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.pos_to_index(GridPos(3, 1)), 13);
    }

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        // Finding symbol locations
        let grid = parse_input(SAMPLE);
        assert_eq!(grid.symbol_locations().collect::<Vec<_>>().len(), 6);
        assert_eq!(grid.symbol_locations().next().unwrap(), GridPos(3, 1));

        // Finding valid part numbers
        let mut nums = grid.part_numbers().iter().map(|x| x.1).collect::<Vec<_>>();
        nums.sort();
        assert_eq!(nums, vec![35, 467, 592, 598, 617, 633, 664, 755]);

        // Checking overall solution
        assert_eq!(format!("{:?}", solve_pt1(grid)), "4361")
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let grid = parse_input(SAMPLE);
        let nums = grid.part_numbers();
        assert_eq!(format!("{:?}", solve_pt2(grid)), "467835")
    }
}
