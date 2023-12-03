use crate::data::load;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
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
    fn is_near_symbol(&self, symbols: &HashSet<Coord>) -> bool {
        for coord in self.locs.iter() {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let neighbor = Coord {
                        r: coord.r + dr,
                        c: coord.c + dc,
                    };
                    if symbols.contains(&neighbor) {
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
    let mut pns = Vec::new();
    let digit_re = Regex::new(r"\d").unwrap();
    let mut pn_comps: Vec<(Coord, char)> = Vec::new();
    for (i, c) in line.chars().enumerate() {
        if digit_re.is_match(&c.to_string()) {
            pn_comps.push((
                Coord {
                    r: *row,
                    c: i as i32,
                },
                c,
            ))
        } else if !pn_comps.is_empty() {
            pns.push(PartNum::from_components(&pn_comps));
            pn_comps = Vec::new();
        }
    }

    if !pn_comps.is_empty() {
        pns.push(PartNum::from_components(&pn_comps));
    }

    Ok(pns)
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
    let (part_nums, syms) = parse_input(input_data)?;
    let sym_coords = HashSet::from_iter(syms.keys().cloned());
    Ok(part_nums
        .iter()
        .filter(|p| p.is_near_symbol(&sym_coords))
        .map(|p| p.val)
        .sum())
}

fn get_neighbors(coord: &Coord, part_nums: &[PartNum]) -> Vec<PartNum> {
    let syms = HashSet::from_iter([*coord]);
    part_nums
        .iter()
        .filter(|p| p.is_near_symbol(&syms))
        .cloned()
        .collect()
}

pub fn puzzle_2(input: &str) -> Result<i32, PuzzleErr> {
    let (part_nums, symbols) = parse_input(input)?;
    Ok(symbols
        .iter()
        .filter(|(_, s)| s == &&'*')
        .map(|(c, _)| {
            let pn = get_neighbors(c, &part_nums);
            if pn.len() == 2 {
                pn[0].val * pn[1].val
            } else {
                0
            }
        })
        .sum())
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
    assert_eq!(answer_2, Ok(72246648))
}
