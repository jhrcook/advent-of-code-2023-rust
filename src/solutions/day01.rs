use crate::data::load;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Failed parsing integer.")]
    ParseIntError(#[from] ParseIntError),
    #[error("Could not locate digit.")]
    NoDigits,
}

fn extract_digits_1(text: &str) -> Vec<isize> {
    lazy_static! {
        static ref DIGIT_REGEX: Regex = Regex::new(r"[0-9]").unwrap();
    }
    DIGIT_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str().parse::<isize>().unwrap())
        .collect()
}

fn extract_digits_2(text: &str) -> Vec<isize> {
    let regex_set = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut digits: Vec<Option<isize>> = Vec::from_iter((0..text.len()).map(|_| None));
    for i in 0..text.len() {
        for (re, val) in regex_set.iter() {
            if text[i..].starts_with(re) {
                digits[i] = Some(*val);
                break;
            }
        }
    }
    digits.iter().flatten().cloned().collect::<Vec<_>>()
}

fn make_calibration_number(digits: &[isize]) -> Result<isize, PuzzleError> {
    Ok(digits.first().ok_or(PuzzleError::NoDigits)? * 10
        + digits.last().ok_or(PuzzleError::NoDigits)?)
}

fn calc_total_calibration(
    input_data: &str,
    extraction_func: fn(&str) -> Vec<isize>,
) -> Result<isize, PuzzleError> {
    let total = input_data
        .trim()
        .lines()
        .map(|a| a.trim())
        .map(|line| make_calibration_number(&extraction_func(line)))
        .collect::<Result<Vec<isize>, PuzzleError>>()?
        .iter()
        .sum();
    Ok(total)
}

pub fn puzzle_1(input_data: &str) -> Result<isize, PuzzleError> {
    calc_total_calibration(input_data, extract_digits_1)
}

pub fn puzzle_2(input_data: &str) -> Result<isize, PuzzleError> {
    calc_total_calibration(input_data, extract_digits_2)
}

pub fn main(data_dir: &str) {
    println!("Day 1: Trebuchet?!");
    let data = load(data_dir, 1, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(56042));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(55358))
}
