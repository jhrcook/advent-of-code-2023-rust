use crate::data::load;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Input parsing error.")]
    ParseInputError,
}

pub fn puzzle_1(input: &str) -> Result<usize, PuzzleErr> {
    Ok(0)
}

pub fn main(data_dir: &str) {
    println!("Day 00: TITLE");
    let data = load(data_dir, 00, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    // assert_eq!(answer_1, Ok(37113));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(30449))
}
