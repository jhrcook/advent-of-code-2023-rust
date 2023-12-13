use crate::data::load;
use itertools::Itertools;
use std::{fmt::Display, iter::zip, num::ParseIntError};
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

// fn display_vec<T>(conditions: &[T]) -> String
// where
//     T: Display,
// {
//     conditions.iter().map(|c| format!("{}", c)).join("")
// }

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

impl Row {
    fn _final_validation(&self, prop_conds: &[Condition]) -> bool {
        let splits = _split_condition_vec(prop_conds);
        if self.groups.len() != splits.len() {
            return false;
        }
        for (seq, gr) in zip(splits.iter(), self.groups.iter()) {
            if seq.len() != *gr {
                return false;
            }
        }
        true
    }

    fn _in_progress_validation(&self, prop_conds: &[Condition]) -> bool {
        let splits = _split_condition_vec(prop_conds);
        if self.groups.len() < splits.len() {
            return false;
        }
        for (i, seq) in splits.iter().enumerate() {
            if seq.len() > self.groups[i] {
                return false;
            }
        }
        true
    }

    fn validate(&self, prop_conds: &Vec<Condition>) -> bool {
        if prop_conds.len() == self.conditions.len() {
            self._final_validation(prop_conds)
        } else {
            self._in_progress_validation(prop_conds)
        }
    }

    fn prune(&self, prop_conditions: &[Vec<Condition>]) -> Vec<Vec<Condition>> {
        prop_conditions
            .iter()
            .filter(|cond| self.validate(cond))
            .cloned()
            .collect::<Vec<_>>()
    }

    fn num_solutions(&self) -> usize {
        let mut prop_conds: Vec<Vec<Condition>> = Vec::new();
        prop_conds.push(Vec::new());
        for cond in self.conditions.iter() {
            match cond {
                Condition::Unknown => {
                    let _prec_prop_conds = prop_conds.clone();
                    prop_conds.clear();
                    for new_cond in [&Condition::Damaged, &Condition::Operational] {
                        for _cond in _prec_prop_conds.iter() {
                            let mut _new_prop_cond = _cond.clone();
                            _new_prop_cond.push(*new_cond);
                            prop_conds.push(_new_prop_cond);
                        }
                    }
                }
                _ => {
                    prop_conds.iter_mut().for_each(|c| c.push(*cond));
                }
            }
            prop_conds = self.prune(&prop_conds);
        }
        prop_conds.len()
    }
}

fn parse_input(input: &str) -> Result<Vec<Row>, PuzzleErr> {
    input
        .trim()
        .lines()
        .map(Row::try_from)
        .collect::<Result<Vec<_>, PuzzleErr>>()
}

pub fn puzzle_1(input: &str) -> Result<usize, PuzzleErr> {
    let rows = parse_input(input)?;
    Ok(rows.into_iter().map(|r| r.num_solutions()).sum())
}

pub fn puzzle_2(input: &str) -> Result<usize, PuzzleErr> {
    let rows = parse_input(input)?
        .iter()
        .map(|r| {
            let mut c = (0..5)
                .map(|_| {
                    let mut new_c = r.conditions.clone();
                    new_c.push(Condition::Unknown);
                    new_c
                })
                .concat();
            let _ = c.pop();
            let g = (0..5).map(|_| r.groups.clone()).concat();
            Row {
                conditions: c,
                groups: g,
            }
        })
        .collect::<Vec<Row>>();

    Ok(rows.into_iter().map(|r| r.num_solutions()).sum())
    // for row in rows.iter() {
    //     log::debug!("{}", row);
    // }
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
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(933))
}
