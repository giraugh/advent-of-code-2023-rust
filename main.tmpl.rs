mod puzzle;

use puzzle::*;
use std::{env, fs};

fn main() {
    // Read input
    let input_path = env::args().nth(1).unwrap_or("input.txt".to_owned());
    let input_text = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Can't find AOC input file {}", &input_path));

    // Parse input
    let input = parse_input(&input_text);

    // Solve and print first part
    let pt1 = solve_pt1(input.clone());
    println!("PT1: {:?}", pt1);

    // Solve and print second part
    let pt2 = solve_pt2(input);
    println!("PT2: {:?}", pt2);
}

#[cfg(test)]
mod test_with_sample {
    use super::puzzle::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");

    #[test]
    fn pt1_sample() {
        let sample_input = parse_input(SAMPLE_TEXT);
        let output = format!("{:?}", solve_pt1(sample_input));
        assert_eq!(&output, SAMPLE_ANS_PT1);
    }

    #[test]
    fn pt2_sample() {
        let sample_input = parse_input(SAMPLE_TEXT);
        let output = format!("{:?}", solve_pt2(sample_input));
        assert_eq!(&output, SAMPLE_ANS_PT2);
    }
}
