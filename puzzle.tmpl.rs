type PuzzleInput = String;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    input_text.to_owned()
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    "todo"
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    "todo"
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt1(input)), todo!());
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), todo!());
    }
}
