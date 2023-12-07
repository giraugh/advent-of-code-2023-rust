use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    convert::Infallible,
    fmt::Debug,
    str::FromStr,
};

use itertools::{repeat_n, Itertools};

#[derive(Clone, Copy, PartialEq, Eq, Hash, derive_more::From, derive_more::Into)]
struct Card(char);

impl Card {
    fn value(&self) -> usize {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            c if c.is_ascii_digit() => c.to_string().parse::<usize>().unwrap(),
            '*' => 1, // Joker
            _ => panic!(),
        }
    }

    fn is_joker(&self) -> bool {
        self.0 == '*'
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card({})", self.0)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Hand([Card; 5]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithJokers(Hand);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn get_type(&self) -> HandType {
        // Find how many cards are the same
        let mut counts: HashMap<Card, usize> = HashMap::new();
        for card in &self.0 {
            counts.entry(*card).and_modify(|c| *c += 1).or_insert(1);
        }

        match counts.values().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                // Does the next biggest group have 2 or 1?
                let next_highest = counts.values().sorted().rev().nth(1).unwrap();
                if *next_highest == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            _ => {
                // check for two pair or one pair or none
                // how many pairs
                let pairs = counts.values().filter(|&&c| c == 2).count();
                match pairs {
                    2 => HandType::TwoPair,
                    1 => HandType::OnePair,
                    _ => HandType::HighCard,
                }
            }
        }
    }

    fn with_jokers(self) -> WithJokers {
        let hand = self
            .0
            .map(|card| if card.0 == 'J' { Card('*') } else { card });
        WithJokers(Hand(hand))
    }

    fn compare_card_by_card(&self, other: &Self) -> Ordering {
        for (card_a, card_b) in self.0.iter().zip(other.0.iter()) {
            if card_a == card_b {
                continue;
            }

            return if card_a > card_b {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Less
            };
        }

        unreachable!();
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hand({})",
            self.0.iter().map(|c| c.0).collect::<String>()
        )
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // If the hands are the same, they are equal
        if self.eq(other) {
            return cmp::Ordering::Equal;
        }

        // Otherwise
        match self.get_type().cmp(&other.get_type()) {
            cmp::Ordering::Less => cmp::Ordering::Less,
            cmp::Ordering::Greater => cmp::Ordering::Greater,
            cmp::Ordering::Equal => self.compare_card_by_card(other),
        }
    }
}

impl WithJokers {
    fn get_type(&self) -> HandType {
        // Find how many cards are the same
        let mut counts: HashMap<Card, usize> = HashMap::new();
        for card in self.0 .0.iter().filter(|c| !c.is_joker()) {
            counts.entry(*card).and_modify(|c| *c += 1).or_insert(1);
        }

        // If all jokers, return best hand
        let joker_count = self.0 .0.iter().filter(|c| c.is_joker()).count();
        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }

        // Add jokers to most prevelant
        let most_prev = counts.iter().max_by_key(|(_k, v)| **v).unwrap().0;
        counts.entry(*most_prev).and_modify(|c| *c += joker_count);

        // Determine type
        match counts.values().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                // Does the next biggest group have 2 or 1?
                let next_highest = counts.values().sorted().rev().nth(1).unwrap();
                if *next_highest == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            _ => {
                // check for two pair or one pair or none
                // how many pairs
                let pairs = counts.values().filter(|&&c| c == 2).count();
                match pairs {
                    2 => HandType::TwoPair,
                    1 => HandType::OnePair,
                    _ => HandType::HighCard,
                }
            }
        }
    }
}

impl PartialOrd for WithJokers {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WithJokers {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // If the hands are the same, they are equal
        if self.eq(other) {
            return cmp::Ordering::Equal;
        }

        // Otherwise
        match self.get_type().cmp(&other.get_type()) {
            cmp::Ordering::Less => cmp::Ordering::Less,
            cmp::Ordering::Greater => cmp::Ordering::Greater,
            cmp::Ordering::Equal => self.0.compare_card_by_card(&other.0),
        }
    }
}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<Card> = s[0..5].chars().map(|c| c.into()).collect();
        Ok(Hand(cards.try_into().unwrap()))
    }
}

type PuzzleInput = Vec<(Hand, usize)>;

/// Parse puzzle input
pub fn parse_input(input_text: &str) -> PuzzleInput {
    input_text
        .lines()
        .map(|l| {
            let (hand, bet) = l.split_once(' ').unwrap();
            (hand.parse().unwrap(), bet.parse().unwrap())
        })
        .collect_vec()
}

/// Solve puzzle part 1
pub fn solve_pt1(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .iter()
        .sorted_by_key(|(hand, _bet)| hand)
        .enumerate()
        .map(|(i, (_hand, bet))| (i + 1) * bet)
        .sum::<usize>()
}

/// Solve puzzle part 2
pub fn solve_pt2(input: PuzzleInput) -> impl std::fmt::Debug {
    input
        .into_iter()
        .map(|(hand, bet)| (hand.with_jokers(), bet))
        .sorted_by_key(|(hand, _bet)| hand.clone())
        .enumerate()
        .map(|(i, (_hand, bet))| (i + 1) * bet)
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
        assert_eq!(format!("{:?}", solve_pt1(input)), "6440");
    }

    /// Test part 2 with sample
    #[test]
    fn part_2_sample() {
        let input = parse_input(SAMPLE_TEXT);
        assert_eq!(format!("{:?}", solve_pt2(input)), "5905");
    }

    #[test]
    fn test_generic_sort() {
        let hands = vec![
            Hand::from_str("KTTTT").unwrap(),
            Hand::from_str("T5555").unwrap(),
            Hand::from_str("QQQQA").unwrap(),
        ];
        for hand in &hands {
            assert_eq!(hand.get_type(), HandType::FourOfAKind);
        }
        let sorted = hands.iter().sorted().collect_vec();
        dbg!(&sorted);
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].0, hands[1].0);
        assert_eq!(sorted[1].0, hands[2].0);
        assert_eq!(sorted[2].0, hands[0].0);
    }

    #[test]
    fn test_joker_cmp() {
        let a = Hand::from_str("JKKK2").unwrap().with_jokers();
        let b = Hand::from_str("QQQQ2").unwrap().with_jokers();
        assert!(a < b)
    }
}
