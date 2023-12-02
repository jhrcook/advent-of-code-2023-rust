use crate::data::load;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Could not parse game: '{}'.", .0)]
    GameParsingError(String),
    #[error("Could not parse game info: '{}'.", .0)]
    GameInfoParsingError(String),
    #[error("Could not parse cube: '{}'.", .0)]
    CubeParsingError(String),
}

#[derive(Debug, Clone)]
enum Cube {
    Blue(u32),
    Red(u32),
    Green(u32),
}

impl Cube {
    fn from_input(input: &str) -> Result<Self, PuzzleError> {
        let Some(caps) = Regex::new(r"(?<n>\d+) (?<color>\w+)")
            .unwrap()
            .captures(input)
        else {
            return Err(PuzzleError::CubeParsingError(input.to_string()));
        };
        let n = &caps["n"].parse::<u32>().unwrap();
        match &caps["color"] {
            "blue" => Ok(Cube::Blue(*n)),
            "red" => Ok(Cube::Red(*n)),
            "green" => Ok(Cube::Green(*n)),
            _ => Err(PuzzleError::CubeParsingError(input.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    info: Vec<Vec<Cube>>,
}

fn parse_game_info_piece(info_str: &str) -> Result<Vec<Cube>, PuzzleError> {
    log::debug!("Parsing info string: {}", info_str);
    info_str
        .trim()
        .split(',')
        .map(|a| Cube::from_input(a.trim()))
        .collect()
}

fn prase_input_line(input_line: &str) -> Result<Game, PuzzleError> {
    let Some(caps) = Regex::new(r"Game (?<id>\d+):")
        .unwrap()
        .captures(input_line)
    else {
        return Err(PuzzleError::GameParsingError(input_line.to_string()));
    };
    let id = &caps["id"].parse::<u32>().unwrap();
    log::debug!("game ID: {}", id);

    let info = input_line.split(": ").collect::<Vec<_>>()[1]
        .split(';')
        .map(parse_game_info_piece)
        .collect::<Result<Vec<Vec<Cube>>, PuzzleError>>()?;

    Ok(Game { id: *id, info })
}

fn parse_input(input_data: &str) -> Result<Vec<Game>, PuzzleError> {
    input_data
        .trim()
        .lines()
        .map(|a| prase_input_line(a.trim()))
        .collect()
}

pub fn puzzle_1(input_data: &str) -> Result<u32, PuzzleError> {
    let games = parse_input(input_data)?;
    let mut tally = 0;
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    'game: for game in games {
        for info in game.info {
            for cube in info {
                let is_possible = match cube {
                    Cube::Blue(n) => n <= 14,
                    Cube::Red(n) => n <= 12,
                    Cube::Green(n) => n <= 13,
                };
                if !is_possible {
                    continue 'game;
                }
            }
        }
        tally += game.id;
    }
    Ok(tally)
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
    // assert_eq!(answer_1, Ok(56042));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(55358))
}
