use std::num::ParseIntError;

use crate::data::load;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Integer parsing error.")]
    Disconnect(#[from] ParseIntError),
}

fn parse_input(data: &str) -> Result<Vec<Vec<i32>>, PuzzleErr> {
    Ok(data
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, ParseIntError>>()
        })
        .collect::<Result<Vec<_>, ParseIntError>>()?)
}

fn oasis_prediction(seq: &[i32]) -> i32 {
    let mut diffs = Vec::from_iter([seq.to_owned()]);
    loop {
        let next_seq = diffs
            .last()
            .unwrap()
            .windows(2)
            .map(|a| a[1] - a[0])
            .collect::<Vec<_>>();
        diffs.push(next_seq.clone());
        if next_seq.iter().all(|a| a == &0) {
            break;
        }
    }

    let mut preds = Vec::from_iter([0]);
    for i in (1..diffs.len()).rev() {
        preds.push(preds.last().unwrap() + diffs[i - 1].last().unwrap());
    }
    *preds.last().unwrap()
}

pub fn puzzle_1(input: &str) -> Result<i32, PuzzleErr> {
    let seqs = parse_input(input)?;
    Ok(seqs.iter().map(|s| oasis_prediction(s)).sum())
}

pub fn main(data_dir: &str) {
    println!("Day 9: Mirage Maintenance");
    let data = load(data_dir, 9, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(1666172641));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(21003205388413))
}
