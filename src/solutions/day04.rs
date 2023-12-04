use crate::data::load;
use linked_hash_set::LinkedHashSet;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]

pub enum PuzzleErr {
    #[error("Could not parse card: '{}'.", .0)]
    CardParsingError(String),
    #[error("Could not parse numbers: '{}'.", .0)]
    NumParsingError(String),
}

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning_nums: LinkedHashSet<u32>,
    observed_nums: LinkedHashSet<u32>,
}

fn parse_nums(nums: &str) -> Result<LinkedHashSet<u32>, PuzzleErr> {
    nums.split(' ')
        .filter(|c| !c.is_empty())
        .map(|x| {
            x.trim()
                .parse::<u32>()
                .or(Err(PuzzleErr::NumParsingError(nums.to_string())))
        })
        .collect()
}

impl Card {
    fn from_input(data: &str, id: u32) -> Result<Self, PuzzleErr> {
        let split_data = data.split(':').collect::<Vec<_>>()[1]
            .trim()
            .split('|')
            .collect::<Vec<_>>();
        if split_data.len() != 2 {
            return Err(PuzzleErr::CardParsingError(data.to_string()));
        }
        Ok(Card {
            id,
            winning_nums: parse_nums(split_data[0])?,
            observed_nums: parse_nums(split_data[1])?,
        })
    }

    fn score(&self) -> u32 {
        match self.n_matches() {
            0 => 0,
            1 => 1,
            x => 2_u32.pow(x - 1),
        }
    }

    fn n_matches(&self) -> u32 {
        self.winning_nums
            .intersection(&self.observed_nums)
            .collect::<Vec<_>>()
            .len() as u32
    }
}

fn parse_input(input: &str) -> Result<Vec<Card>, PuzzleErr> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, t)| Card::from_input(t, i as u32 + 1))
        .collect::<Result<_, _>>()
}

pub fn puzzle_1(input: &str) -> Result<u32, PuzzleErr> {
    Ok(parse_input(input)?.iter().map(|c| c.score()).sum())
}

fn count_cards(cards: &HashMap<u32, Card>, count: &mut HashMap<u32, u32>) {
    for i in (1_u32..=cards.len() as u32).rev() {
        let card = cards.get(&i).unwrap();
        let new_card_ids: Vec<u32> = match card.n_matches() {
            0 => Vec::new(),
            x => Vec::from_iter((card.id + 1)..=(card.id + x)),
        };
        let num_new_cards: u32 = new_card_ids
            .iter()
            .map(|i| count.get(i).unwrap())
            .sum::<u32>()
            + new_card_ids.len() as u32;
        count.insert(card.id, num_new_cards);
    }
}

pub fn puzzle_2(input: &str) -> Result<u32, PuzzleErr> {
    let cards = parse_input(input)?
        .iter()
        .map(|c| (c.id, c.clone()))
        .collect::<HashMap<_, _>>();
    let mut card_count: HashMap<u32, u32> = HashMap::new();
    count_cards(&cards, &mut card_count);
    Ok(card_count.values().sum::<u32>() + cards.len() as u32)
}

pub fn main(data_dir: &str) {
    println!("Day 4: Scratchcards");
    let data = load(data_dir, 4, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(21158));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(6050769))
}
