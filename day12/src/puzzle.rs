use std::{collections::HashMap, sync::RwLock};

use itertools::Itertools;
use lazy_static::lazy_static;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};
use tqdm::Iter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for SpringCondition {
    fn from(value: char) -> Self {
        match value {
            '#' => SpringCondition::Damaged,
            '.' => SpringCondition::Operational,
            '?' => SpringCondition::Unknown,
            c => panic!("Unrecognised spring condition: {c}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    springs: Vec<SpringCondition>,
    groups: Vec<usize>,
}

// Initialise a global cache for use in Record::possible_arrangements_with()
// the cache key is just the args to that function (might be able to simplify I think)
type CacheKey = (Vec<SpringCondition>, Vec<usize>, bool);
type Cache = HashMap<CacheKey, usize>;
lazy_static! {
    static ref CACHE: RwLock<Cache> = RwLock::new(HashMap::new());
}

impl Record {
    fn parse(input: &str) -> IResult<&str, Self> {
        // Parse springs
        let (input, springs) = terminated(
            many1(alt((
                value(SpringCondition::Damaged, tag("#")),
                value(SpringCondition::Operational, tag(".")),
                value(SpringCondition::Unknown, tag("?")),
            ))),
            tag(" "),
        )(input)?;

        // Parse groups
        let (input, groups) =
            all_consuming(separated_list1(tag(","), map(u32, |x| x as usize)))(input)?;

        Ok((input, Self { springs, groups }))
    }

    /// Recursively find the number of possible arrangements given a current set of spring
    /// conditions, current groups and whether currently in the middle of a # span
    ///
    /// the rough approach is to read the spring conditions one at a time. Then I updated the
    /// current required group count. When I hit a question mark it recursively forks for both options.
    /// If at any point the current condition would cause the goals to be invalid, it zeros out
    /// that branch.
    fn possible_arrangements_with(
        springs: &[SpringCondition],
        groups: &[usize],
        in_span: bool,
    ) -> usize {
        // In cache?
        let key = (springs.into(), groups.into(), in_span);
        if let Some(v) = CACHE.read().unwrap().get(&key) {
            return *v;
        }

        // Evaluate
        let value = match springs.first() {
            // No springs left, was it valid in the end?
            None => match groups.len() {
                0 => 1,
                1 if groups.first() == Some(&0) => 1,
                _ => 0,
            },

            // This spring is operational. This ends the current span.
            // IF the span wasn't ended already and we hadn't finished the current group
            // then zero out this branch
            Some(&SpringCondition::Operational) => {
                let (_, tail) = springs.split_at(1);
                match groups.first() {
                    Some(0) => {
                        let (_, groups_tail) = groups.split_at(1);
                        Self::possible_arrangements_with(tail, groups_tail, false)
                    }
                    Some(_) => {
                        if in_span {
                            0
                        } else {
                            Self::possible_arrangements_with(tail, groups, in_span)
                        }
                    }
                    None => Self::possible_arrangements_with(tail, groups, in_span),
                }
            }

            // This spring is damaged. This starts a span if not already started and decrements
            // the current group count. If we had already finished the group then this branch is
            // invalid
            Some(&SpringCondition::Damaged) => {
                let (_, tail) = springs.split_at(1);
                let mut groups: Vec<_> = groups.into();
                match groups.first() {
                    None => 0,
                    Some(0) => 0,
                    Some(_) => {
                        groups[0] -= 1;
                        Self::possible_arrangements_with(tail, &groups, true)
                    }
                }
            }

            // This spring is either broken or operational. Try both and add the
            // possible arrangements with either. If either is invalid then there will be
            // zero possible arrangements.
            Some(&SpringCondition::Unknown) => {
                let (_, tail) = springs.split_at(1);
                let a = {
                    let mut springs_v: Vec<_> = tail.into();
                    springs_v.insert(0, SpringCondition::Damaged);
                    Self::possible_arrangements_with(&springs_v, groups, in_span)
                };
                let b = {
                    let mut springs_v: Vec<_> = tail.into();
                    springs_v.insert(0, SpringCondition::Operational);
                    Self::possible_arrangements_with(&springs_v, groups, in_span)
                };
                a + b
            }
        };

        // Write to cache and return value
        CACHE.write().unwrap().insert(key, value);
        value
    }

    /// Entry point for recursively finding number of possible arrangements
    fn possible_arrangements(&mut self) -> usize {
        Self::possible_arrangements_with(&self.springs, &self.groups, false)
    }

    /// Expand and then get possible arrangements for this record
    fn expanded_possible_arrangements(&self, expanded_factor: usize) -> usize {
        // Expand self into longer record
        let springs = (0..expanded_factor).map(|_| self.springs.clone());
        let springs = itertools::Itertools::intersperse(springs, vec![SpringCondition::Unknown])
            .flatten()
            .collect_vec();
        let groups = (0..expanded_factor)
            .flat_map(|_| self.groups.clone())
            .collect_vec();
        let mut expanded = Self { groups, springs };

        // Solve expanded record
        expanded.possible_arrangements()
    }
}

type PuzzleInput = Vec<Record>;

/// Parse puzzle input
pub fn parse_input(input: &str) -> PuzzleInput {
    input.lines().map(|l| Record::parse(l).unwrap().1).collect()
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .into_iter()
        .map(|mut record| record.possible_arrangements())
        .sum::<usize>()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .into_iter()
        .tqdm()
        .map(|record| record.expanded_possible_arrangements(5))
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
        assert_eq!(format!("{:?}", solve_pt1(input)), "21");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "525152");
    }

    macro_rules! arrs {
        ($s: expr, $v: expr) => {
            assert_eq!(
                Record::parse($s).unwrap().1.possible_arrangements(),
                $v,
                "Expected {} arrs for {}",
                $v,
                $s
            )
        };
    }

    #[test]
    fn simple_test() {
        arrs!("???.### 1,1,3", 1);
        arrs!(".??..??...?##. 1,1,3", 4);
        arrs!("?#?#?#?#?#?#?#? 1,3,1,6", 1);
        arrs!("????.#...#... 4,1,1", 1);
        arrs!("????.######..#####. 1,6,5", 4);
        arrs!("?###???????? 3,2,1", 10);
    }
}
