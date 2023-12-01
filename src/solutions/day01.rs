use crate::data::load;
use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError<'a> {
    #[error("Failed parsing integer.")]
    ParseIntError(#[from] ParseIntError),
    #[error("Could not locate digit: {}.", .0)]
    NoDigit(&'a str),
}

fn extract_digits(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref DIGIT_REGEX: Regex = Regex::new(r"[0-9]").unwrap();
    }
    DIGIT_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str())
        .collect()
}

pub fn puzzle_1(input_data: &str) -> Result<isize, PuzzleError> {
    let mut total = 0;
    for line in input_data.trim().lines().map(|a| a.trim()) {
        let digits_str = extract_digits(line);
        let number: isize = match format!(
            "{}{}",
            digits_str.first().ok_or(PuzzleError::NoDigit(line))?,
            digits_str.last().ok_or(PuzzleError::NoDigit(line))?
        )
        .parse()
        {
            Ok(x) => x,
            Err(e) => return Err(PuzzleError::ParseIntError(e)),
        };
        total += number
    }
    Ok(total)
}

pub fn main(data_dir: &str) {
    println!("Day 1: Trebuchet?!");
    let data = load(data_dir, 1, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        _ => panic!("No solution to puzzle 1."),
    }
    // assert_eq!(answer_1, Ok(68787));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     _ => panic!("No solution to puzzle 2."),
    // }
    // assert_eq!(answer_2, Ok(198041))
}

#[cfg(test)]
mod tests {
    use crate::solutions::day01::puzzle_1;

    const EXAMPLE_INPUT: &str = "
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    #[test]
    fn example_1_puzzle_1() {
        assert_eq!(puzzle_1(self::EXAMPLE_INPUT), Ok(142));
    }
}
