use itertools::Itertools;

type PuzzleInput = Vec<String>;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    input_text
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|s| s.to_owned())
        .collect()
}

fn hash(s: &str) -> usize {
    s.chars()
        .map(|c| c as usize)
        .fold(0, |acc, v| ((acc + v) * 17) % 256)
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[derive(Clone, Debug)]
enum Operation {
    Remove(String),
    Insert(String, usize),
}

type Boxes = [Vec<Lens>; 256];

impl Operation {
    fn parse(input: &str) -> Self {
        if let Some((label, focal_length)) = input.split_once('=') {
            Self::Insert(label.to_owned(), focal_length.parse().unwrap())
        } else if let Some(label) = input.strip_suffix('-') {
            Self::Remove(label.to_owned())
        } else {
            unreachable!()
        }
    }
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> usize {
    input.iter().map(|s| hash(s)).sum()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    let mut boxes: Boxes = std::array::from_fn(|_| Default::default());
    for op in input.into_iter().map(|s| Operation::parse(&s)) {
        match op {
            Operation::Remove(label) => {
                let bkx = &mut boxes[hash(&label)];
                if let Some((index, _)) = bkx.iter().find_position(|l| l.label == label) {
                    bkx.remove(index);
                }
            }
            Operation::Insert(label, focal_length) => {
                let bkx = &mut boxes[hash(&label)];
                match bkx.iter().find_position(|l| l.label == label) {
                    Some((index, _)) => {
                        bkx[index].focal_length = focal_length;
                    }
                    None => {
                        bkx.push(Lens {
                            label,
                            focal_length,
                        });
                    }
                }
            }
        }
    }

    // Return focusing power
    boxes
        .into_iter()
        .enumerate()
        .map(|(box_index, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(|(lens_index, lens)| (box_index + 1) * (lens_index + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt1(input)), "1320");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "145");
    }
}
