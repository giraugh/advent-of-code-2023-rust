use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, u64},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use tqdm::Iter;

type Category = String;

#[allow(unused)]
#[derive(Debug, Clone)]
struct CategoryMap {
    from: Category,
    to: Category,
    map_ranges: Vec<(u64, u64, u64)>,
}

#[derive(Debug, Clone)]
pub struct Almanac {
    initial_seeds: Vec<u64>,
    maps: Vec<CategoryMap>,
}

fn double_newline(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((newline, newline))(input)?;
    Ok((input, ()))
}

impl CategoryMap {
    /// Parse a category map name and ranges
    fn parse(input: &str) -> IResult<&str, Self> {
        // Parse category from/to
        let (input, (from, to)) = separated_pair(alpha1, tag("-to-"), alpha1)(input)?;
        let (from, to) = (from.to_owned(), to.to_owned());

        // Parse `map:`
        let (input, _) = tag(" map:\n")(input)?;

        // Parse ranges
        let (input, map_ranges) = separated_list1(
            newline,
            tuple((terminated(u64, tag(" ")), terminated(u64, tag(" ")), u64)),
        )(input)?;

        Ok((
            input,
            Self {
                from,
                to,
                map_ranges,
            },
        ))
    }

    fn forward(&self, value: &u64) -> u64 {
        for &(to_start, from_start, len) in &self.map_ranges {
            if (from_start..from_start + len).contains(value) {
                let delta = value - from_start;
                return to_start + delta;
            }
        }
        *value
    }

    fn backward(&self, value: &u64) -> u64 {
        for &(to_start, from_start, len) in &self.map_ranges {
            if (to_start..to_start + len).contains(value) {
                let delta = value - to_start;
                return from_start + delta;
            }
        }
        *value
    }
}

impl Almanac {
    /// Parse an entire almanac
    fn parse(input: &str) -> IResult<&str, Self> {
        // Parse the initial seeds
        let (input, initial_seeds) = preceded(
            tag("seeds: "),
            terminated(separated_list1(tag(" "), u64), double_newline),
        )(input)?;

        // parse each map
        let (input, maps) = separated_list1(double_newline, CategoryMap::parse)(input)?;

        // Parse trailing newline
        let (input, _) = all_consuming(newline)(input)?;

        Ok((
            input,
            Self {
                maps,
                initial_seeds,
            },
        ))
    }

    /// take a seed value and pass it through all maps
    /// to get a location
    fn through_all(&self, seed: u64) -> u64 {
        let mut seed = seed;
        for category_map in &self.maps {
            seed = category_map.forward(&seed);
        }
        seed
    }

    /// take a location value and pass it backwards through all maps
    /// to get a seed
    fn back_through_all(&self, location: u64) -> u64 {
        let mut location = location;
        for category_map in self.maps.iter().rev() {
            location = category_map.backward(&location);
        }
        location
    }
}

type PuzzleInput = Almanac;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    Almanac::parse(input_text).unwrap().1
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    // take each initial seed, then parse through every map to
    // get the final locations
    input
        .initial_seeds
        .iter()
        .map(|seed| input.through_all(*seed))
        .min()
        .unwrap()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    (0..1_000_000_000) // lol
        .tqdm()
        .find(|&location| {
            // Get possible seed by going backwards
            let possible_seed = input.back_through_all(location);
            debug_assert_eq!(input.through_all(possible_seed), location);

            input
                .initial_seeds
                .chunks(2)
                .map(|l| l[0]..l[0] + l[1])
                .any(|r| r.contains(&possible_seed))
        })
        .unwrap()
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
        assert_eq!(format!("{:?}", solve_pt1(input)), "35");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "46");
    }
}
