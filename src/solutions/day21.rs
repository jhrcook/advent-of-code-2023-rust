use crate::data::load;
use std::{collections::HashSet, hash::Hash};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Input parsing error.")]
    ParseInputError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    r: i32,
    c: i32,
}

impl Pos {
    fn new(r: i32, c: i32) -> Self {
        Self { r, c }
    }

    fn move_by(&self, dr: &i32, dc: &i32) -> Self {
        Self {
            r: self.r + dr,
            c: self.c + dc,
        }
    }
}

struct GardenMap {
    start: Pos,
    garden_plots: HashSet<Pos>,
}

impl GardenMap {
    fn neighboring_garden_plots(&self, p: &Pos) -> HashSet<Pos> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(dr, dc)| p.move_by(dr, dc))
            .filter(|p| self.garden_plots.contains(p))
            .collect()
    }
}

fn parse_input(input: &str) -> GardenMap {
    let mut start: Option<Pos> = None;
    let mut garden_plots = HashSet::new();
    for (r, line) in input.trim().lines().enumerate() {
        for (c, x) in line.trim().chars().enumerate() {
            match x {
                '.' => {
                    garden_plots.insert(Pos::new(r as i32 + 1, c as i32 + 1));
                }
                'S' => {
                    garden_plots.insert(Pos::new(r as i32 + 1, c as i32 + 1));
                    start = Some(Pos::new(r as i32 + 1, c as i32 + 1));
                }
                _ => (),
            }
        }
    }
    GardenMap {
        start: start.unwrap(),
        garden_plots,
    }
}

pub fn puzzle_1(input: &str, n_steps: u32) -> Result<usize, PuzzleErr> {
    let map = parse_input(input);
    let mut current_positions: HashSet<Pos> = HashSet::from_iter([map.start]);
    for _ in 0..n_steps {
        current_positions = current_positions
            .iter()
            .flat_map(|p| map.neighboring_garden_plots(p))
            .collect();
    }
    Ok(current_positions.len())
}

pub fn main(data_dir: &str) {
    println!("Day 21: Step Counter");
    let data = load(data_dir, 21, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data, 64);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    // assert_eq!(answer_1, Ok(37113));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(30449))
}
