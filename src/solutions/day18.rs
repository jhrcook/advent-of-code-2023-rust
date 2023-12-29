use crate::data::load;
use num::integer::div_floor;
use regex::Regex;
use std::i64;
use std::{iter::zip, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Input parsing error: {}.", .0)]
    ParseInputError(String),
    #[error("Integer parsing error.")]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl TryFrom<&char> for Direction {
    type Error = PuzzleErr;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Direction::U),
            'D' => Ok(Direction::D),
            'L' => Ok(Direction::L),
            'R' => Ok(Direction::R),
            _ => Err(PuzzleErr::ParseInputError(value.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
struct Dig {
    dir: Direction,
    n: i64,
}

fn line_to_dig(line: &str) -> Result<Dig, PuzzleErr> {
    let re = Regex::new(r"^(?<dir>\w{1}) (?<n>\d+)").unwrap();
    let Some(caps) = re.captures(line) else {
        return Err(PuzzleErr::ParseInputError(line.trim().to_string()));
    };
    Ok(Dig {
        dir: Direction::try_from(&caps["dir"].chars().next().unwrap())?,
        n: caps["n"].parse::<i64>()?,
    })
}

fn line_to_dig_2(line: &str) -> Result<Dig, PuzzleErr> {
    let re = Regex::new(r"\(\#(?<color>.+)\)").unwrap();
    let Some(caps) = re.captures(line) else {
        return Err(PuzzleErr::ParseInputError(line.trim().to_string()));
    };
    let color = caps["color"].to_string();
    let dir = match color.chars().last().unwrap() {
        '0' => Direction::R,
        '1' => Direction::D,
        '2' => Direction::L,
        '3' => Direction::U,
        c => panic!("Unknown char: {}", c),
    };
    let n = i64::from_str_radix(&color.as_str()[..5], 16).unwrap();
    Ok(Dig { dir, n })
}

fn parse_input(
    input: &str,
    line_parse_func: &dyn Fn(&str) -> Result<Dig, PuzzleErr>,
) -> Result<Vec<Dig>, PuzzleErr> {
    input
        .trim()
        .lines()
        .map(line_parse_func)
        .collect::<Result<Vec<_>, PuzzleErr>>()
}

fn move_pos(p: &(i64, i64), dig: &Dig) -> (i64, i64) {
    match dig.dir {
        Direction::U => (p.0 + dig.n, p.1),
        Direction::D => (p.0 - dig.n, p.1),
        Direction::L => (p.0, p.1 - dig.n),
        Direction::R => (p.0, p.1 + dig.n),
    }
}

fn dig_plan_to_vertices(dig_plan: &[Dig]) -> Vec<(i64, i64)> {
    let mut vertices = Vec::from_iter([(0, 0)]);
    for dig in dig_plan.iter() {
        let a = vertices.last().unwrap();
        let b = move_pos(a, dig);
        vertices.push(b);
    }
    vertices
}

fn shoelace(vertices: &[(i64, i64)]) -> i64 {
    let a: i64 = zip(vertices.iter(), vertices[1..].iter())
        .map(|(a, b)| (a.0 * b.1) - (a.1 * b.0))
        .sum();
    a / 2
}

fn perimeter(vertices: &[(i64, i64)]) -> i64 {
    zip(vertices.iter(), vertices[1..].iter())
        .map(|(a, b)| ((((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f32).sqrt()) as i64)
        .sum()
}

pub fn puzzle_1(input: &str) -> Result<i64, PuzzleErr> {
    let dig_plan = parse_input(input, &line_to_dig)?;
    let vertices = dig_plan_to_vertices(&dig_plan);
    Ok(shoelace(&vertices) + div_floor(perimeter(&vertices), 2) + 1)
}

pub fn puzzle_2(input: &str) -> Result<i64, PuzzleErr> {
    let dig_plan = parse_input(input, &line_to_dig_2)?;
    let vertices = dig_plan_to_vertices(&dig_plan);
    Ok(shoelace(&vertices) + div_floor(perimeter(&vertices), 2) + 1)
}

pub fn main(data_dir: &str) {
    println!("Day 18: Lavaduct Lagoon");
    let data = load(data_dir, 18, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(72821));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(127844509405501))
}
