use crate::data::load;
use std::{cmp::Ordering, collections::HashMap, iter::zip};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr<'a> {
    #[error("Input parsing error: '{}'.", .0)]
    InputParsingError(&'a str),
    #[error("Unrecognized card.")]
    UnknownCard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_str(s: char) -> Result<Self, PuzzleErr<'static>> {
        match s {
            '2' => Ok(Self::N2),
            '3' => Ok(Self::N3),
            '4' => Ok(Self::N4),
            '5' => Ok(Self::N5),
            '6' => Ok(Self::N6),
            '7' => Ok(Self::N7),
            '8' => Ok(Self::N8),
            '9' => Ok(Self::N9),
            'T' => Ok(Self::T),
            'J' => Ok(Self::J),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            _ => Err(PuzzleErr::UnknownCard),
        }
    }
}

fn count<T: std::hash::Hash + std::cmp::Eq + Copy>(v: &[T]) -> HashMap<T, u32> {
    let mut map = HashMap::new();
    for x in v.iter() {
        map.entry(*x).and_modify(|a| *a += 1_u32).or_insert(1_u32);
    }
    map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32) -> Self {
        Self { cards, bid }
    }

    fn hand_type(&self) -> HandType {
        let card_count = count(&self.cards);
        match card_count.len() {
            1 => HandType::FiveKind,
            2 => match card_count.values().max().unwrap() {
                3 => HandType::FullHouse,
                4 => HandType::FourKind,
                _ => panic!("Logic error in card_count = 2."),
            },
            3 => match card_count.values().max().unwrap() {
                3 => HandType::ThreeKind,
                2 => HandType::TwoPair,
                _ => panic!("Logic error in card_count = 3."),
            },
            4 => HandType::OnePair,
            5 => HandType::High,
            _ => panic!("Too many cards in a hand."),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cards == other.cards {
            return Ordering::Equal;
        }
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                for (a, b) in zip(&self.cards, &other.cards) {
                    if a != b {
                        return a.cmp(b);
                    }
                }
                panic!("Logic error in hand comparison.")
            }
            cmp_result => cmp_result,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

fn _line_to_hand(line: &str) -> Result<Hand, PuzzleErr> {
    let split_str = line.split_whitespace().collect::<Vec<_>>();
    let cards = split_str[0]
        .chars()
        .map(Card::from_str)
        .collect::<Result<Vec<_>, PuzzleErr>>()?;
    let bid = split_str[1]
        .parse::<u32>()
        .or(Err(PuzzleErr::InputParsingError(line)))?;
    Ok(Hand::new(cards, bid))
}

fn parse_input(input: &str) -> Result<Vec<Hand>, PuzzleErr> {
    input.trim().lines().map(_line_to_hand).collect()
}

pub fn puzzle_1(input: &str) -> Result<u32, PuzzleErr> {
    let mut cards = parse_input(input)?;
    cards.sort();
    return Ok(cards
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum());
}

pub fn main(data_dir: &str) {
    println!("Day 7: Camel Cards");
    let data = load(data_dir, 7, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(254024898));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(27363861))
}
