use crate::data::load;
use regex::Regex;
use std::cmp::max;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not parse game: '{}'.", .0)]
    GameParsingError(String),
    #[error("Could not parse game info: '{}'.", .0)]
    GameInfoParsingError(String),
    #[error("Could not parse cube: '{}'.", .0)]
    CubeParsingError(String),
}

#[derive(Debug, Clone)]
enum Cube {
    R(u32),
    B(u32),
    G(u32),
}

impl Cube {
    fn from_input(input: &str) -> Result<Self, PuzzleErr> {
        let Some(caps) = Regex::new(r"(?<n>\d+) (?<color>\w+)")
            .unwrap()
            .captures(input)
        else {
            return Err(PuzzleErr::CubeParsingError(input.to_string()));
        };
        let n = caps["n"].parse::<u32>().unwrap();
        match &caps["color"] {
            "red" => Ok(Cube::R(n)),
            "blue" => Ok(Cube::B(n)),
            "green" => Ok(Cube::G(n)),
            _ => Err(PuzzleErr::CubeParsingError(input.to_string())),
        }
    }

    fn n(&self) -> u32 {
        match self {
            Cube::R(n) => *n,
            Cube::B(n) => *n,
            Cube::G(n) => *n,
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    info: Vec<Vec<Cube>>,
}

impl Game {
    fn minimum_cubes(&self) -> Vec<Cube> {
        let (mut min_b, mut min_r, mut min_g) = (0, 0, 0);
        for cube in self.info.iter().flatten() {
            match cube {
                Cube::R(n) => min_r = max(*n, min_r),
                Cube::B(n) => min_b = max(*n, min_b),
                Cube::G(n) => min_g = max(*n, min_g),
            };
        }
        Vec::from_iter([Cube::B(min_b), Cube::R(min_r), Cube::G(min_g)])
    }
}

fn parse_game_info_piece(info_str: &str) -> Result<Vec<Cube>, PuzzleErr> {
    info_str
        .trim()
        .split(',')
        .map(|a| Cube::from_input(a.trim()))
        .collect()
}

fn prase_input_line(input_line: &str) -> Result<Game, PuzzleErr> {
    let Some(caps) = Regex::new(r"Game (?<id>\d+):")
        .unwrap()
        .captures(input_line)
    else {
        return Err(PuzzleErr::GameParsingError(input_line.to_string()));
    };
    let id = &caps["id"].parse::<u32>().unwrap();

    let info = input_line.split(": ").collect::<Vec<_>>()[1]
        .split(';')
        .map(parse_game_info_piece)
        .collect::<Result<Vec<Vec<Cube>>, PuzzleErr>>()?;

    Ok(Game { id: *id, info })
}

fn parse_input(input_data: &str) -> Result<Vec<Game>, PuzzleErr> {
    input_data
        .trim()
        .lines()
        .map(|a| prase_input_line(a.trim()))
        .collect()
}

pub fn puzzle_1(input_data: &str) -> Result<u32, PuzzleErr> {
    let games = parse_input(input_data)?;
    let mut s = 0;
    'game: for game in games {
        for info in game.info {
            for cube in info {
                let is_possible = match cube {
                    Cube::R(n) => n <= 12,
                    Cube::B(n) => n <= 14,
                    Cube::G(n) => n <= 13,
                };
                if !is_possible {
                    continue 'game;
                }
            }
        }
        s += game.id;
    }
    Ok(s)
}

pub fn puzzle_2(input_data: &str) -> Result<u32, PuzzleErr> {
    Ok(parse_input(input_data)?
        .iter()
        .map(|g| g.minimum_cubes().iter().map(|c| c.n()).product::<u32>())
        .sum())
}

pub fn main(data_dir: &str) {
    println!("Day 2: Cube Conundrum");
    let data = load(data_dir, 2, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(2679));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(77607))
}
