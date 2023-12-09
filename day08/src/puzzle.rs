use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(u32)]
enum Turn {
    Left = 0,
    Right = 1,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node(String);

impl Node {
    pub fn new(s: &str) -> Self {
        Self(s.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Network {
    dirs: Vec<Turn>,
    nodes: HashMap<Node, (Node, Node)>,
}

impl Network {
    fn parse(input: &str) -> IResult<&str, Self> {
        // Parse dirs
        let (input, dirs) = terminated(
            many1(alt((
                value(Turn::Left, tag("L")),
                value(Turn::Right, tag("R")),
            ))),
            tuple((newline, newline)),
        )(input)?;

        // Parse network
        let (input, nodes) = separated_list1(
            newline,
            tuple((
                terminated(alphanumeric1, tag(" = ")),
                delimited(
                    tag("("),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    tag(")"),
                ),
            )),
        )(input)?;

        let nodes = nodes
            .into_iter()
            .map(|(a, (b, c))| (Node::new(a), (Node::new(b), Node::new(c))))
            .collect::<HashMap<_, _>>();

        Ok((input, Self { nodes, dirs }))
    }

    /// Find the length of the path from AAA to ZZZ
    fn path_length<F>(&self, from: &str, targ_pred: F) -> usize
    where
        F: Fn(&Node) -> bool,
    {
        let mut position = Node::new(from);
        let mut dirs = self.dirs.iter().cycle();
        let mut steps = 0;
        while !targ_pred(&position) {
            let dir = dirs.next().unwrap();
            let (left, right) = self.nodes.get(&position).unwrap();
            position = if *dir == Turn::Left {
                left.clone()
            } else {
                right.clone()
            };
            steps += 1;
        }
        steps
    }

    /// Find the length of the path from **A to **Z for all **
    fn ghost_path_length(&self) -> usize {
        // Find starting positions
        let positions: Vec<Node> = self
            .nodes
            .keys()
            .filter(|k| k.0.ends_with('A'))
            .cloned()
            .collect();

        // For each starting position, measure the path length
        // the ghost path length is the lcm of them
        positions
            .iter()
            .map(|p| self.path_length(&p.0, |p| p.0.ends_with('Z')))
            .reduce(lcm)
            .unwrap()
    }
}

type PuzzleInput = Network;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    Network::parse(input_text).unwrap().1
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    input.path_length("AAA", |p| p.0 == "ZZZ")
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    input.ghost_path_length()
}

/// Calculate greatest common divisor
fn gcd(a: usize, b: usize) -> usize {
    if a > 0 {
        gcd(b % a, a)
    } else {
        b
    }
}

/// Calculate least common multiple
fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");
    const SAMPLE_2_TEXT: &str = include_str!("../sample2.txt");

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt1(input)), "2");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_2_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "6");
    }
}
