use crate::data::load;
use linked_hash_set::LinkedHashSet;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]

pub enum PuzzleErr {
    #[error("Could not parse card: '{}'.", .0)]
    CardParsingError(String),
    #[error("Could not parse numbers: '{}'.", .0)]
    NumParsingError(String),
}

struct Card {
    winning_nums: LinkedHashSet<u32>,
    observed_nums: LinkedHashSet<u32>,
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 5

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
    fn from_input(data: &str) -> Result<Self, PuzzleErr> {
        let split_data = data.split(':').collect::<Vec<_>>()[1]
            .trim()
            .split('|')
            .collect::<Vec<_>>();
        if split_data.len() != 2 {
            return Err(PuzzleErr::CardParsingError(data.to_string()));
        }
        Ok(Card {
            winning_nums: parse_nums(split_data[0])?,
            observed_nums: parse_nums(split_data[1])?,
        })
    }

    fn score(&self) -> u32 {
        match self
            .winning_nums
            .intersection(&self.observed_nums)
            .collect::<Vec<_>>()
            .len()
        {
            0 => 0,
            1 => 1,
            x => 2_u32.pow((x - 1) as u32),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Card>, PuzzleErr> {
    input
        .trim()
        .lines()
        .map(Card::from_input)
        .collect::<Result<_, _>>()
}

pub fn puzzle_1(input: &str) -> Result<u32, PuzzleErr> {
    Ok(parse_input(input)?.iter().map(|c| c.score()).sum())
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
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(72246648))
}
