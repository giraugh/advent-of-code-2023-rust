use std::{
    collections::{BinaryHeap, HashMap},
    rc::Rc,
    sync::RwLock,
};

use aoc::{
    direction::Dir,
    grid::{Grid, GridPos},
    pos,
};

#[derive(Debug, Clone)]
pub struct City(Grid<usize>);

impl City {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        Self(Grid::new(grid))
    }

    fn min_heat(&self, min_before_turn: usize, max_before_turn: usize) -> usize {
        // Get path
        let mut path = PathSearch::new(self, pos!(0, 0), pos!(self.0.width - 1, self.0.height - 1))
            .find_path(min_before_turn, max_before_turn)
            .unwrap();

        self.0
            .print_cells(|p, _| if path.contains(&p) { '#' } else { '.' });

        // Pop the start position (we dont count it)
        path.pop();

        // Count the heat levels for each path position
        path.into_iter().map(|p| self.0.get(p).unwrap()).sum()
    }
}

#[derive(Debug, Clone)]
struct SearchNode<'a> {
    pos: GridPos,
    previous_same_dirs: Vec<Dir>,
    search: Rc<RwLock<PathSearch<'a>>>,
    f_score: usize,
}

impl SearchNode<'_> {
    fn backtrack(&self) -> Vec<GridPos> {
        let search = self.search.read().unwrap();
        let mut parents = vec![self.pos];
        let mut parent = search.parents.get(self);
        while let Some(p) = parent {
            parents.push(p.pos);
            parent = search.parents.get(p);
        }

        parents
    }
}

impl std::hash::Hash for SearchNode<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.previous_same_dirs.hash(state);
    }
}

impl PartialEq for SearchNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.previous_same_dirs == other.previous_same_dirs
    }
}

impl PartialOrd for SearchNode<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_score.cmp(&other.f_score).reverse()
    }
}

impl Eq for SearchNode<'_> {}

#[derive(Debug)]
struct PathSearch<'a> {
    city: &'a City,
    start_pos: GridPos,
    target_pos: GridPos,
    parents: HashMap<Rc<SearchNode<'a>>, Rc<SearchNode<'a>>>,
    g_scores: HashMap<Rc<SearchNode<'a>>, usize>,
}

impl<'a> PathSearch<'a> {
    fn new(city: &'a City, start_pos: GridPos, target_pos: GridPos) -> Self {
        Self {
            city,
            start_pos,
            target_pos,
            parents: Default::default(),
            g_scores: Default::default(),
        }
    }

    fn find_path(self, min_before_turn: usize, max_before_turn: usize) -> Option<Vec<GridPos>> {
        let mut frontier = BinaryHeap::new();

        let search = Rc::new(RwLock::new(self));

        let start = Rc::new(SearchNode {
            pos: search.read().unwrap().start_pos,
            previous_same_dirs: vec![],
            f_score: 0,
            search: search.clone(),
        });

        search.write().unwrap().g_scores.insert(start.clone(), 0);
        frontier.push(start);

        while let Some(state) = frontier.pop() {
            // Is this the goal?
            if state.pos == search.read().unwrap().target_pos
                && state.previous_same_dirs.len() > min_before_turn
            {
                return Some(state.backtrack());
            }

            // Expand
            state
                .pos
                .neighbours()
                .filter(|pos| pos.in_grid(&search.read().unwrap().city.0))
                .filter_map(|pos| {
                    let dir = Dir::try_from(pos - state.pos).unwrap();

                    // If same, keep the dirs
                    let previous_dir = state.previous_same_dirs.last();

                    // Can't go backwards
                    if previous_dir == Some(&dir.opposite()) {
                        return None;
                    }

                    // If this is too few in same dirs, we can't do it
                    // not sure about this
                    if previous_dir.is_some() && Some(&dir) != previous_dir {
                        // turned
                        if state.previous_same_dirs.len() < min_before_turn {
                            return None;
                        }
                    }

                    let previous_same_dirs = if previous_dir == Some(&dir) {
                        let mut pd = state.previous_same_dirs.clone();
                        pd.push(dir);
                        pd
                    } else {
                        vec![dir]
                    };

                    // If this is too many in same dirs, we cant do it at all
                    if previous_same_dirs.len() == max_before_turn + 1 {
                        return None;
                    }

                    Some(Rc::new(SearchNode {
                        pos,
                        previous_same_dirs,
                        search: search.clone(),
                        f_score: 0,
                    }))
                })
                .for_each(|child_state| {
                    let mut search = search.write().unwrap();
                    let state_g = search.g_scores.get(&state).unwrap();
                    let edge_cost = search.city.0.get(child_state.pos).unwrap();
                    let tentative_g = state_g + edge_cost;
                    if search
                        .g_scores
                        .get(&child_state)
                        .map(|&existing_g| tentative_g < existing_g)
                        .unwrap_or(true)
                    {
                        search.parents.insert(child_state.clone(), state.clone());
                        search.g_scores.insert(child_state.clone(), tentative_g);
                        if !frontier.as_slice().contains(&child_state) {
                            let h_score = 0;
                            frontier.push(Rc::new(SearchNode {
                                pos: child_state.pos,
                                previous_same_dirs: child_state.previous_same_dirs.clone(),
                                f_score: tentative_g + h_score,
                                search: child_state.search.clone(),
                            }))
                        }
                    }
                })
        }

        // Didn't find a path
        None
    }
}

type PuzzleInput = City;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    City::parse(input_text)
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> usize {
    input.min_heat(0, 3)
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    input.min_heat(4, 10)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt1(input)), "102");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "94");
    }
}
