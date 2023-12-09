use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, i64},
    combinator::map,
    multi::separated_list1,
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct History(Vec<i64>);

type PuzzleInput = Vec<History>;

impl History {
    /// Parse a history from a line
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(tag(" "), i64), History)(input)
    }

    // Find sequences down to 0s
    fn fill(&self) -> Vec<Vec<i64>> {
        let mut seqs = vec![self.0.clone()];
        let mut curr_seq = self.0.clone();
        loop {
            let diffs: Vec<i64> = curr_seq
                .clone()
                .into_iter()
                .map_windows(|[a, b]| b - a)
                .collect();

            if diffs.iter().all(|&s| s == 0) {
                break;
            } else {
                seqs.push(diffs.clone());
                curr_seq = diffs;
            }
        }

        seqs
    }

    /// Extrapolate to get next value in the sequence
    fn extrapolate(&self) -> i64 {
        let mut seqs = self.fill();

        // Go back up adding values
        for i in (0..seqs.len()).rev() {
            // Get last value from previous row if applicable
            let prev = if let Some(prev_seq) = seqs.get(i + 1) {
                *prev_seq.last().unwrap()
            } else {
                0
            };

            // Add previous value to value at end of current sequence
            let curr_seq = &mut seqs[i];
            let last = curr_seq.last().unwrap();
            curr_seq.push(last + prev);
        }

        *seqs[0].last().unwrap()
    }

    fn extrapolate_backwards(&self) -> i64 {
        let mut seqs = self.fill();

        // Go back up subtracting values
        for i in (0..seqs.len()).rev() {
            // Get first value from previous row if applicable
            let prev = if let Some(prev_seq) = seqs.get(i + 1) {
                *prev_seq.first().unwrap()
            } else {
                0
            };

            // Add previous value to value at end of current sequence
            let curr_seq = &mut seqs[i];
            let first = curr_seq.first().unwrap();
            curr_seq.insert(0, first - prev);
        }

        *seqs[0].first().unwrap()
    }
}

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    input_text
        .lines()
        .map(|l| History::parse(l).unwrap().1)
        .collect()
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .iter()
        .map(|history| history.extrapolate())
        .sum::<i64>()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .iter()
        .map(|history| history.extrapolate_backwards())
        .sum::<i64>()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt1(input)), "114");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "2");
    }
}
