use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

type PuzzleInput = Races;

#[derive(Debug, Clone)]
pub struct Races(Vec<Race>);

#[derive(Debug, Clone)]
pub struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    /// Button holding durations that will beat the record
    fn winning_button_press_durations(&self) -> Vec<u64> {
        // dist(bt, rt) = (rt - bt) * bt
        (0..=self.time)
            .filter(|button_duration| {
                let dist = (self.time - button_duration) * button_duration;
                dist > self.record_distance
            })
            .collect()
    }
}

impl Races {
    fn reinterpret_as_kerning(self) -> Race {
        let time = self
            .0
            .iter()
            .map(|race| race.time.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let record_distance = self
            .0
            .iter()
            .map(|race| race.record_distance.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        Race {
            time,
            record_distance,
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (times, record_distances)) = separated_pair(
            preceded(
                terminated(tag("Time: "), space1),
                separated_list1(space1, u64),
            ),
            newline,
            preceded(
                terminated(tag("Distance: "), space1),
                separated_list1(space1, u64),
            ),
        )(input)?;

        let races = times
            .iter()
            .zip(record_distances.iter())
            .map(|(&time, &record_distance)| Race {
                time,
                record_distance,
            })
            .collect();

        Ok((input, Self(races)))
    }
}

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    Races::parse(input_text).unwrap().1
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .0
        .iter()
        .map(|race| race.winning_button_press_durations().len())
        .product::<usize>()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    let race = input.reinterpret_as_kerning();
    race.winning_button_press_durations().len()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = include_str!("../sample.txt");

    /// Test part 1 with sample
    #[test]
    fn part_1_sample() {
        let input = parse_input(SAMPLE_TEXT);
        dbg!(&input);
        assert_eq!(format!("{:?}", solve_pt1(input)), "288");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "71503");
    }
}
