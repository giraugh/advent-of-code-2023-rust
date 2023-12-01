use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::{map_res, peek, value},
    multi::{many1, many_till},
    sequence::terminated,
    IResult,
};

type PuzzleInput = String;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    input_text.to_owned()
}

/// parse a "digit" token, if found return the token and the remaining string
/// consumes only a single character as a bit of a hack to allow overlapping tokens
#[rustfmt::skip]
fn parse_digit_token(input: &str) -> IResult<&str, usize> {
    alt((
        // Parse a digit name
        value(1, terminated(tag("o"), peek(tag("ne")))),
        value(2, terminated(tag("t"), peek(tag("wo")))),
        value(3, terminated(tag("t"), peek(tag("hree")))),
        value(4, terminated(tag("f"), peek(tag("our")))),
        value(5, terminated(tag("f"), peek(tag("ive")))),
        value(6, terminated(tag("s"), peek(tag("ix")))),
        value(7, terminated(tag("s"), peek(tag("even")))),
        value(8, terminated(tag("e"), peek(tag("ight")))),
        value(9, terminated(tag("n"), peek(tag("ine")))),

        // Or parse a single digit
        map_res(satisfy(|c| c.is_ascii_digit()), |c| c.to_string().parse()),
    ))(input)
}

/// Parse concatenated digit tokens with optional chaff between them
fn parse_digit_tokens(input: &str) -> IResult<&str, Vec<usize>> {
    many1(nom::combinator::map(
        many_till(satisfy(|c| !c.is_ascii_digit()), parse_digit_token),
        |(_, d)| d,
    ))(input)
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .lines()
        .map(|l| {
            let digits = l.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>();
            let (first, last) = (digits.first().unwrap(), digits.last().unwrap());
            format!("{}{}", first, last).parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .lines()
        .map(|l| {
            let (_, digits) = parse_digit_tokens(l).unwrap();
            let (first, last) = (digits.first().unwrap(), digits.last().unwrap());
            format!("{}{}", first, last).parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_digits() {
        let s = "one2three4five";
        let (_, digits) = parse_digit_tokens(s).unwrap();
        assert_eq!(digits, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_digits_overlap() {
        let s = "oneight";
        let (_, digits) = parse_digit_tokens(s).unwrap();
        assert_eq!(digits, [1, 8]);
    }

    #[test]
    fn test_parse_digits_chaff() {
        let s = "oneadw2bbffgthreeaa4hhfive";
        let (_, digits) = parse_digit_tokens(s).unwrap();
        assert_eq!(digits, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_digits_again() {
        let s = "fourfive4tttldbmmkxvhqrmvmrkpxfzbd7";
        let (_, digits) = parse_digit_tokens(s).unwrap();
        assert_eq!(digits, [4, 5, 4, 7]);
    }

    #[test]
    fn test_pt2_sample() {
        let s = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let o = solve_pt2(s.to_owned());
        assert_eq!(format!("{:?}", o), "281");
    }
}
