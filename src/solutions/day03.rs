use crate::data::load;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not parse game: '{}'.", .0)]
    GameParsingError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    r: i32,
    c: i32,
}

#[derive(Debug, Clone)]
struct PartNum {
    val: i32,
    locs: Vec<Coord>,
}

impl PartNum {
    fn is_near_symbol(&self, symbol_locs: &HashMap<Coord, char>) -> bool {
        for coord in self.locs.iter() {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let neighbor = Coord {
                        r: coord.r + dr,
                        c: coord.c + dc,
                    };
                    if symbol_locs.contains_key(&neighbor) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn from_components(part_num_comps: &[(Coord, char)]) -> Self {
        let val = part_num_comps
            .iter()
            .map(|(_, c)| c)
            .join("")
            .parse::<i32>()
            .unwrap();
        PartNum {
            val,
            locs: part_num_comps.iter().map(|(c, _)| c).cloned().collect(),
        }
    }
}

fn extract_part_nums_from_line(line: &str, row: &i32) -> Result<Vec<PartNum>, PuzzleErr> {
    let mut part_nums = Vec::new();
    let digit_regex = Regex::new(r"\d").unwrap();
    let mut part_num_comps: Vec<(Coord, char)> = Vec::new();
    for (i, c) in line.chars().enumerate() {
        if digit_regex.is_match(&c.to_string()) {
            part_num_comps.push((
                Coord {
                    r: *row,
                    c: i as i32,
                },
                c,
            ))
        } else if !part_num_comps.is_empty() {
            part_nums.push(PartNum::from_components(&part_num_comps));
            part_num_comps = Vec::new();
        }
    }

    if !part_num_comps.is_empty() {
        part_nums.push(PartNum::from_components(&part_num_comps));
    }

    Ok(part_nums)
}

fn extract_symbols(input_data: &str) -> Result<HashMap<Coord, char>, PuzzleErr> {
    let mut symbols = HashMap::new();
    let digit_regex = Regex::new(r"\d").unwrap();
    for (r, line) in input_data.trim().lines().enumerate() {
        for (c, sym) in line.trim().chars().enumerate() {
            if (sym == '.') | (digit_regex.is_match(&sym.to_string())) {
                continue;
            }
            symbols.insert(
                Coord {
                    r: r as i32,
                    c: c as i32,
                },
                sym,
            );
        }
    }
    Ok(symbols)
}

fn parse_input(input_data: &str) -> Result<(Vec<PartNum>, HashMap<Coord, char>), PuzzleErr> {
    let part_nums = input_data
        .trim()
        .lines()
        .enumerate()
        .map(|(r, txt)| extract_part_nums_from_line(txt.trim(), &(r as i32)))
        .flatten_ok()
        .collect::<Result<Vec<_>, PuzzleErr>>()?;
    let symbols = extract_symbols(input_data)?;
    Ok((part_nums, symbols))
}

pub fn puzzle_1(input_data: &str) -> Result<i32, PuzzleErr> {
    let (part_nums, symbols) = parse_input(input_data)?;
    Ok(part_nums
        .iter()
        .filter(|p| p.is_near_symbol(&symbols))
        .map(|p| p.val)
        .sum())
}

pub fn puzzle_2(input_data: &str) -> Result<i32, PuzzleErr> {
    Ok(0)
}

pub fn main(data_dir: &str) {
    println!("Day 3: Gear Ratios");
    let data = load(data_dir, 3, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(498559));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    // assert_eq!(answer_2, Ok(77607))
}
