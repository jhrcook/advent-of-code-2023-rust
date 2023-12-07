use crate::data::load;
use std::{cmp::Ordering, collections::HashMap, hash::Hash, iter::zip};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr<'a> {
    #[error("Input parsing error: '{}'.", .0)]
    InputParsingError(&'a str),
    #[error("Unrecognized card.")]
    UnknownCard,
}

trait FromStr {
    fn from_str(s: char) -> Result<Self, PuzzleErr<'static>>
    where
        Self: Sized;
}

trait Card: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash + FromStr {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card1 {
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

impl Card for Card1 {}

impl FromStr for Card1 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card2 {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

impl FromStr for Card2 {
    fn from_str(s: char) -> Result<Self, PuzzleErr<'static>> {
        match Card1::from_str(s) {
            Ok(Card1::N2) => Ok(Self::N2),
            Ok(Card1::N3) => Ok(Self::N3),
            Ok(Card1::N4) => Ok(Self::N4),
            Ok(Card1::N5) => Ok(Self::N5),
            Ok(Card1::N6) => Ok(Self::N6),
            Ok(Card1::N7) => Ok(Self::N7),
            Ok(Card1::N8) => Ok(Self::N8),
            Ok(Card1::N9) => Ok(Self::N9),
            Ok(Card1::T) => Ok(Self::T),
            Ok(Card1::J) => Ok(Self::J),
            Ok(Card1::Q) => Ok(Self::Q),
            Ok(Card1::K) => Ok(Self::K),
            Ok(Card1::A) => Ok(Self::A),
            Err(e) => Err(e),
        }
    }
}

impl Card for Card2 {}

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
struct Hand<C: Card> {
    cards: Vec<C>,
    bid: u32,
}

impl<C: Card> Hand<C> {
    fn new(cards: Vec<C>, bid: u32) -> Self {
        Self { cards, bid }
    }
}

trait GetHandType {
    fn hand_type(&self) -> HandType;
}

fn _card_count_to_hand_type<T: core::hash::Hash>(counts: &HashMap<T, u32>) -> HandType {
    match counts.len() {
        1 => HandType::FiveKind,
        2 => match counts.values().max().unwrap() {
            3 => HandType::FullHouse,
            4 => HandType::FourKind,
            _ => panic!("Logic error in card_count = 2."),
        },
        3 => match counts.values().max().unwrap() {
            3 => HandType::ThreeKind,
            2 => HandType::TwoPair,
            _ => panic!("Logic error in card_count = 3."),
        },
        4 => HandType::OnePair,
        5 => HandType::High,
        _ => panic!("Too many cards in a hand."),
    }
}

impl GetHandType for Hand<Card1> {
    fn hand_type(&self) -> HandType {
        let card_count = count(&self.cards);
        _card_count_to_hand_type(&card_count)
    }
}

impl GetHandType for Hand<Card2> {
    fn hand_type(&self) -> HandType {
        let mut card_count = count(&self.cards);
        if card_count.len() == 1 {
            return HandType::FiveKind;
        }
        if let Some(num_jokers) = card_count.clone().get(&Card2::J) {
            card_count.remove(&Card2::J);
            let max_count = card_count.values().max().unwrap();
            let top_cards = card_count
                .iter()
                .filter(|(_, v)| v == &max_count)
                .map(|(k, _)| k)
                .collect::<Vec<_>>();
            let top_card = top_cards.first().unwrap();
            card_count
                .entry(**top_card)
                .and_modify(|x| *x += num_jokers);
        }
        _card_count_to_hand_type(&card_count)
    }
}

impl<C: Card> Ord for Hand<C>
where
    Hand<C>: GetHandType,
{
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

impl<C: Card> PartialOrd for Hand<C>
where
    Hand<C>: GetHandType,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: Card> PartialEq for Hand<C> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl<C: Card> Eq for Hand<C> {}

fn _line_to_hand<C: Card>(line: &str) -> Result<Hand<C>, PuzzleErr>
where
    C: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash,
{
    let split_str = line.split_whitespace().collect::<Vec<_>>();
    let cards = split_str[0]
        .chars()
        .map(C::from_str)
        .collect::<Result<Vec<_>, PuzzleErr>>()?;
    let bid = split_str[1]
        .parse::<u32>()
        .or(Err(PuzzleErr::InputParsingError(line)))?;
    Ok(Hand::new(cards, bid))
}

fn parse_input<C: Card>(input: &str) -> Result<Vec<Hand<C>>, PuzzleErr> {
    input.trim().lines().map(_line_to_hand).collect()
}

fn score_hands<C: Card>(cards: &mut [Hand<C>]) -> u32
where
    Hand<C>: GetHandType,
{
    cards.sort();
    cards
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum()
}

pub fn puzzle_1(input: &str) -> Result<u32, PuzzleErr> {
    Ok(score_hands(&mut parse_input::<Card1>(input)?))
}

pub fn puzzle_2(input: &str) -> Result<u32, PuzzleErr> {
    Ok(score_hands(&mut parse_input::<Card2>(input)?))
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
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(254115617))
}
