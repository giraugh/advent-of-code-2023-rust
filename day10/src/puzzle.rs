use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
};

use derive_more::{Add, From, Into, Sub};
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Add, Sub, Into, From, Hash)]
struct Pos(isize, isize);

impl Pos {
    /// Get cartesian neighbours of this position
    /// (not guaranteed to be in bounds)
    fn neighbours(&self) -> impl Iterator<Item = Pos> {
        let (x, y) = (self.0, self.1);
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .map(|(x, y)| Pos(x, y))
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Empty,
}

impl Pipe {
    #[rustfmt::skip]
    fn to_subgrid(&self) -> [[u8; 3]; 3] {
        match self {
            Pipe::Vertical =>    [[0, 1, 0],
                                  [0, 1, 0],
                                  [0, 1, 0]],
            Pipe::Horizontal =>  [[0, 0, 0],
                                  [1, 1, 1],
                                  [0, 0, 0]],
            Pipe::NorthToEast => [[0, 1, 0],
                                  [0, 0, 1],
                                  [0, 0, 0]],
            Pipe::NorthToWest => [[0, 1, 0],
                                  [1, 0, 0],
                                  [0, 0, 0]],
            Pipe::SouthToWest => [[0, 0, 0],
                                  [1, 0, 0],
                                  [0, 1, 0]],
            Pipe::SouthToEast => [[0, 0, 0],
                                  [0, 0, 1],
                                  [0, 1, 0]],
            Pipe::Empty =>       [[0, 0, 0],
                                  [0, 0, 0],
                                  [0, 0, 0]],
        }
    }

    /// Is this pipe traversable for someone arriving from `dir` direction?
    pub fn traversable_from_dir(&self, dir: &Pos) -> bool {
        use Pipe::*;

        match dir {
            // West
            Pos(-1, 0) => matches!(self, Horizontal | NorthToWest | SouthToWest),

            // East
            Pos(1, 0) => matches!(self, Horizontal | NorthToEast | SouthToEast),

            // South
            Pos(0, 1) => matches!(self, Vertical | SouthToWest | SouthToEast),

            // North
            Pos(0, -1) => matches!(self, Vertical | NorthToWest | NorthToEast),

            _ => panic!("Invalid direction"),
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Pipe::*;

        match value {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'L' => Ok(NorthToEast),
            'J' => Ok(NorthToWest),
            '7' => Ok(SouthToWest),
            'F' => Ok(SouthToEast),
            '.' => Ok(Empty),
            'S' => Ok(Empty),
            c => Err(format!("Unknown pipe {c}")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pipes {
    pipe_grid: Vec<Vec<Pipe>>,
    starting_pos: Pos,
    width: usize,
    height: usize,
}

impl Pipes {
    fn parse(input: &str) -> Self {
        // Get grid of chars
        let mut starting_pos = (0, 0);
        let pipe_grid = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        // Did we find the start point?
                        if c == 'S' {
                            starting_pos = (x as isize, y as isize);
                        }

                        Pipe::try_from(c).unwrap()
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self {
            height: pipe_grid.len(),
            width: pipe_grid[0].len(),
            starting_pos: starting_pos.into(),
            pipe_grid,
        }
    }

    /// Note: panics if out of bounds
    fn get(&self, pos: Pos) -> Pipe {
        let (x, y) = (pos.0 as usize, pos.1 as usize);
        self.pipe_grid[y][x].clone()
    }

    fn neighbours_from(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        let (w, h) = (self.width as isize, self.height as isize);
        pos.neighbours()
            .filter(move |&Pos(x, y)| x >= 0 && x < w && y >= 0 && y < h)
    }

    /// Positions that can be reached from a given pipe position
    fn traversable_from(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        let this_pipe = self.get(pos);
        self.neighbours_from(pos).filter(move |&pipe_pos| {
            let pipe = self.get(pipe_pos);
            let dir_to = pipe_pos - pos;
            let dir_from = pos - pipe_pos;
            let can_receive = pipe.traversable_from_dir(&dir_from);
            let can_reach = this_pipe.traversable_from_dir(&dir_to);
            can_receive && (can_reach || this_pipe == Pipe::Empty)
        })
    }

    /// Finds the main loop (the one that starts on S) with distances from S for each
    fn find_main_loop(&self) -> HashMap<Pos, usize> {
        // We want to do a breadth first fill that remembers depths
        let mut frontier = VecDeque::new();
        let mut visited: HashMap<Pos, usize> = HashMap::new();
        frontier.push_front((0, self.starting_pos));

        while let Some((depth, pos)) = frontier.pop_front() {
            visited.insert(pos, depth);
            self.traversable_from(pos)
                .filter(|p| {
                    visited
                        .get(p)
                        .map(|&pos_depth| pos_depth > depth)
                        .unwrap_or(true)
                })
                .map(|pos| (depth + 1, pos))
                .for_each(|p| frontier.push_back(p))
        }

        visited
    }

    /// Find the tile on the main loop thats the furthest from the starting position
    fn get_furthest_distance(&self) -> usize {
        *self.find_main_loop().values().max().unwrap()
    }

    fn fill_start_tile(&mut self) {
        // get tiles traversable from start
        let traversable = self.traversable_from(self.starting_pos).collect_vec();

        // infer start tile using traversable
        let Pos(sx, sy) = self.starting_pos;
        let on_east = traversable.contains(&pos!(sx + 1, sy));
        let on_west = traversable.contains(&pos!(sx - 1, sy));
        let on_north = traversable.contains(&pos!(sx, sy - 1));
        let on_south = traversable.contains(&pos!(sx, sy + 1));
        let pipe = match (on_east, on_west, on_north, on_south) {
            (true, true, false, false) => Pipe::Horizontal,
            (false, false, true, true) => Pipe::Vertical,

            (true, false, true, false) => Pipe::NorthToEast,
            (false, true, true, false) => Pipe::NorthToWest,

            (true, false, false, true) => Pipe::SouthToEast,
            (false, true, false, true) => Pipe::SouthToWest,

            _ => panic!(),
        };

        // eprintln!("Set as {pipe:?}");
        self.pipe_grid[sy as usize][sx as usize] = pipe;
    }

    /// Find the area enclosed by the main loop
    fn find_enclosed_area(&mut self) -> usize {
        // Get main loop
        let main_loop = self.find_main_loop();
        let loop_positions: HashSet<_> = main_loop.keys().cloned().collect();

        // Fill in start tile
        self.fill_start_tile();

        // Create higher res mask
        let subgrid_mask: Vec<Vec<[[u8; 3]; 3]>> = self
            .pipe_grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, cell)| {
                        (if loop_positions.contains(&pos!(x, y)) {
                            cell
                        } else {
                            &Pipe::Empty
                        })
                        .to_subgrid()
                    })
                    .collect_vec()
            })
            .collect_vec();

        let subgrid_mask = Subgrid(subgrid_mask);

        // Seed the flood fill at the edges
        let ver_edges = (0..self.width).flat_map(|x| vec![pos!(x, 0), pos!(x, self.height - 1)]);
        let hor_edges = (0..self.height).flat_map(|y| vec![pos!(0, y), pos!(self.width - 1, y)]);

        let mut visited: HashSet<Pos> = HashSet::new();
        visited.extend(
            ver_edges
                .chain(hor_edges)
                .filter(|p| !loop_positions.contains(p))
                .flat_map(|p| {
                    (0..3)
                        .cartesian_product(0..3)
                        .map(move |(x, y)| Pos(p.0 * 3 + x, p.1 * 3 + y))
                }),
        );

        let mut frontier: Vec<_> = visited.iter().cloned().collect();
        while let Some(pos) = frontier.pop() {
            // Track presence
            if !visited.contains(&pos) {
                visited.insert(pos);
            }

            // Add children
            frontier.extend(
                pos.neighbours()
                    .filter(|p| subgrid_mask.inbounds(p))
                    .filter(|p| !subgrid_mask.filled(p))
                    .filter(|p| !visited.contains(p)),
            );
        }

        // Fun output :)
        for y in 0..self.height * 3 {
            for x in 0..self.width * 3 {
                if visited.contains(&pos!(x, y)) {
                    print!("*");
                } else if subgrid_mask.filled(&pos!(x, y)) {
                    print!("o");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        let xs = 0..self.width;
        let ys = 0..self.height;
        xs.cartesian_product(ys)
            .map(|(x, y)| pos!(x, y))
            .filter(|p| {
                (0..3)
                    .cartesian_product(0..3)
                    .all(|(sx, sy)| !visited.contains(&pos!(p.0 * 3 + sx, p.1 * 3 + sy)))
            })
            .count()
    }
}

struct Subgrid(Vec<Vec<[[u8; 3]; 3]>>);

impl Subgrid {
    fn filled(&self, pos: &Pos) -> bool {
        let (x, y) = (pos.0 / 3, pos.1 / 3);
        let (sx, sy) = (pos.0 % 3, pos.1 % 3);
        debug_assert!((0..3).contains(&sx), "sx is out of range {sx}");
        debug_assert!((0..3).contains(&sy), "sy is out of range {sy}");
        self.0[y as usize][x as usize][sy as usize][sx as usize] == 1
    }

    fn inbounds(&self, pos: &Pos) -> bool {
        (0..self.width() as isize).contains(&pos.0) && (0..self.height() as isize).contains(&pos.1)
    }

    fn height(&self) -> usize {
        self.0.len() * 3
    }

    fn width(&self) -> usize {
        self.0[0].len() * 3
    }
}

type PuzzleInput = Pipes;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    Pipes::parse(input_text)
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    input.get_furthest_distance()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    let mut input = input;
    input.find_enclosed_area()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");
    const SAMPLE_2_TEXT: &str = include_str!("../sample2.txt");
    const SAMPLE_3_TEXT: &str = include_str!("../sample3.txt");

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(SAMPLE_TEXT);
        dbg!(&input);
        assert_eq!(format!("{:?}", solve_pt1(input)), "8");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_2_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "4");
    }

    /// Test part 2 with other sample
    #[test]
    fn part_2_sample_2() {
        let input = parse_input(SAMPLE_3_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "8");
    }
}
