use crate::data::load;
use std::{collections::HashSet, fmt::Display};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Input parsing error.")]
    ParseInputError,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    r: isize,
    c: isize,
}

impl Coord {
    fn new(r: isize, c: isize) -> Self {
        Coord { r, c }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.r, self.c)
    }
}

#[derive(Debug, Clone, Default)]
struct Rocks {
    round: HashSet<Coord>,
    square: HashSet<Coord>,
    height: isize,
    width: isize,
}

impl Display for Rocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = (0..self.height)
            .map(|r| {
                (0..self.width)
                    .map(|c| {
                        let coord = Coord::new(r, c);
                        if self.round.contains(&coord) {
                            "O"
                        } else if self.square.contains(&coord) {
                            return "#";
                        } else {
                            return ".";
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", s)
    }
}

fn parse_input(input: &str) -> Rocks {
    let mut rocks = Rocks::default();

    let lines = input.trim().lines().collect::<Vec<_>>();
    rocks.height = lines.len() as isize;
    rocks.width = lines.first().unwrap().len() as isize;

    lines.into_iter().enumerate().for_each(|(r, line)| {
        line.trim().chars().enumerate().for_each(|(c, item)| {
            let coord = Coord::new(r as isize, c as isize);
            match item {
                'O' => rocks.round.insert(coord),
                '#' => rocks.square.insert(coord),
                _ => false,
            };
        })
    });

    rocks
}

fn find_north_position(coord: &Coord, rocks: &Rocks) -> Coord {
    let mut prev_coord = *coord;
    for r in (0..coord.r).rev() {
        let next_coord = Coord::new(r, coord.c);
        if rocks.square.contains(&next_coord) | rocks.round.contains(&next_coord) {
            break;
        }
        prev_coord = next_coord;
    }
    prev_coord
}

fn tilt_north(rocks: &mut Rocks) {
    for r in 0..rocks.height {
        for c in 0..rocks.width {
            let coord = Coord::new(r, c);
            if rocks.round.contains(&coord) {
                rocks.round.remove(&coord);
                rocks.round.insert(find_north_position(&coord, rocks));
            }
        }
    }
}

fn calc_total_load(rocks: &Rocks) -> isize {
    rocks.round.iter().map(|coord| rocks.height - coord.r).sum()
}

pub fn puzzle_1(input: &str) -> isize {
    let mut rocks = parse_input(input);
    tilt_north(&mut rocks);
    calc_total_load(&rocks)
}

pub fn main(data_dir: &str) {
    println!("Day 14: Parabolic Reflector Dish");
    let data = load(data_dir, 14, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    println!(" Puzzle 1: {}", answer_1);
    assert_eq!(answer_1, 112046);

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(30449))
}
