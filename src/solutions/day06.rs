use crate::data::load;
use std::iter::zip;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr<'a> {
    #[error("Input data error: '{}'.", .0)]
    InputDataError(&'a str),
    #[error("Line parsing error: '{}'.", .0)]
    LineParsingError(&'a str),
}

struct Race {
    duration: u32,
    record_distance: u32,
}

impl Race {
    fn n_ways_to_win(&self) -> u32 {
        (1..self.duration)
            .filter(|t| self.record_distance < (t * (self.duration - t)))
            .collect::<Vec<_>>()
            .len() as u32
    }
}

fn _get_nums(line: &str) -> Result<Vec<u32>, PuzzleErr> {
    line.split(':')
        .nth(1)
        .ok_or(PuzzleErr::LineParsingError(line))?
        .split_whitespace()
        .map(|s| s.parse::<u32>().or(Err(PuzzleErr::LineParsingError(line))))
        .collect::<Result<_, _>>()
}

fn parse_data(input: &str) -> Result<Vec<Race>, PuzzleErr> {
    let lines = input.trim().lines().collect::<Vec<_>>();
    if lines.len() != 2 {
        return Err(PuzzleErr::InputDataError("Not two lines in input."));
    }
    let times = _get_nums(lines[0])?;
    let distances = _get_nums(lines[1])?;
    Ok(zip(times, distances)
        .map(|(duration, record_distance)| Race {
            duration,
            record_distance,
        })
        .collect())
}

pub fn puzzle_1(input: &str) -> Result<u32, PuzzleErr> {
    Ok(parse_data(input)?
        .iter()
        .map(|r| r.n_ways_to_win())
        .product())
}

pub fn main(data_dir: &str) {
    println!("Day 6: Wait For It");
    let data = load(data_dir, 6, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    // assert_eq!(answer_1, Ok(650599855));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(1240035))
}
