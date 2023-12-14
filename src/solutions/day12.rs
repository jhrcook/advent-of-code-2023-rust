use crate::data::load;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::{fmt::Display, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Unknown pipe character: {}.", .0)]
    UnknownChar(String),
    #[error("Integer parsing error.")]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<&char> for Condition {
    type Error = PuzzleErr;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Condition::Operational),
            '#' => Ok(Condition::Damaged),
            '?' => Ok(Condition::Unknown),
            _ => Err(PuzzleErr::UnknownChar(value.to_string())),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::Damaged => write!(f, "#"),
            Condition::Operational => write!(f, "."),
            Condition::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone)]
struct Row {
    conditions: Vec<Condition>,
    groups: Vec<usize>,
}

impl TryFrom<&str> for Row {
    type Error = PuzzleErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split_input = value.split_whitespace().collect::<Vec<_>>();
        let spring_conditions = split_input[0]
            .chars()
            .map(|c| Condition::try_from(&c))
            .collect::<Result<Vec<_>, PuzzleErr>>()?;
        let groups = split_input[1]
            .split(',')
            .map(|c| {
                c.to_string()
                    .parse::<usize>()
                    .map_err(PuzzleErr::ParseIntError)
            })
            .collect::<Result<Vec<_>, PuzzleErr>>()?;
        Ok(Self {
            conditions: spring_conditions,
            groups,
        })
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let conditions = self.conditions.iter().map(|c| format!("{}", c)).join("");
        let groups = self.groups.iter().join(", ");
        write!(f, "{}  --  {}", conditions, groups)
    }
}

fn _split_condition_vec(conditions: &[Condition]) -> Vec<&[Condition]> {
    conditions
        .split(|x| x == &Condition::Operational)
        .filter(|cs| !cs.is_empty())
        .collect::<Vec<_>>()
}

fn parse_input(input: &str) -> Result<Vec<Row>, PuzzleErr> {
    input
        .trim()
        .lines()
        .map(Row::try_from)
        .collect::<Result<Vec<_>, PuzzleErr>>()
}

fn add_operational(record: Vec<Condition>, groups: Vec<usize>) -> usize {
    count_solutions(record[1..].to_vec(), groups)
}

fn add_damaged(record: Vec<Condition>, groups: Vec<usize>, next_group: usize) -> usize {
    // This current group must be all "#" or "?" (no "." allowed).
    if record.len() < next_group {
        return 0;
    }
    if record[..next_group]
        .iter()
        .any(|c| c == &Condition::Operational)
    {
        return 0;
    }

    // If the remaining record is the length of the group, then successful if there are
    // no more remaining groups.
    if record.len() == next_group {
        if groups.len() == 1 {
            return 1;
        } else {
            return 0;
        }
    }

    // The next character after the group cannot be "#".
    match record[next_group] {
        Condition::Damaged => 0,
        _ => count_solutions(record[(next_group + 1)..].to_vec(), groups[1..].to_vec()),
    }
}

#[cached]
fn count_solutions(record: Vec<Condition>, groups: Vec<usize>) -> usize {
    // If there are no more groups and no more "#", then successful.
    let Some(next_group) = groups.first() else {
        if record.iter().any(|c| c == &Condition::Damaged) {
            return 0;
        } else {
            return 1;
        }
    };

    // If there are no more items left in record, then unsuccessful.
    let Some(next_cond) = record.first() else {
        return 0;
    };

    match next_cond {
        Condition::Damaged => add_damaged(record.clone(), groups.clone(), *next_group),
        Condition::Operational => add_operational(record.clone(), groups.clone()),
        Condition::Unknown => {
            add_damaged(record.clone(), groups.clone(), *next_group)
                + add_operational(record.clone(), groups.clone())
        }
    }
}

pub fn puzzle_1(input: &str) -> Result<usize, PuzzleErr> {
    Ok(parse_input(input)?
        .iter()
        .map(|r| count_solutions(r.conditions.clone(), r.groups.clone()))
        .sum())
}

fn parse_and_expand_input(input: &str, n_reps: usize) -> Result<Vec<Row>, PuzzleErr> {
    Ok(parse_input(input)?
        .iter()
        .map(|r| {
            let mut c = (0..n_reps)
                .map(|_| {
                    let mut new_c = r.conditions.clone();
                    new_c.push(Condition::Unknown);
                    new_c
                })
                .concat();
            let _ = c.pop();
            let g = (0..n_reps).map(|_| r.groups.clone()).concat();
            Row {
                conditions: c,
                groups: g,
            }
        })
        .collect::<Vec<Row>>())
}

pub fn puzzle_2(input: &str) -> Result<usize, PuzzleErr> {
    Ok(parse_and_expand_input(input, 5)?
        .iter()
        .map(|r| count_solutions(r.conditions.clone(), r.groups.clone()))
        .sum())
    // Ok(0)
}

pub fn main(data_dir: &str) {
    println!("Day 12: Hot Springs");
    let data = load(data_dir, 12, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(7716));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(18716325559999))
}
