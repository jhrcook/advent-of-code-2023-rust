use crate::data::load;
use itertools::Itertools;
use ndarray::{prelude::*, Zip};
use std::{cmp, iter::zip};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Input parsing error.")]
    ParseInputError,
}

fn parse_grid(grid_str: &str) -> Result<Array2<bool>, PuzzleErr> {
    let bools = grid_str
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Ok(false),
                    '#' => Ok(true),
                    _ => Err(PuzzleErr::ParseInputError),
                })
                .collect::<Result<Vec<_>, PuzzleErr>>()
        })
        .collect::<Result<Vec<_>, PuzzleErr>>()?;
    let h = bools.len();
    let w = bools.first().unwrap().len();
    Ok(Array2::from_shape_vec((h, w), bools.concat()).unwrap())
}

fn parse_input(input: &str) -> Result<Vec<Array2<bool>>, PuzzleErr> {
    input
        .trim()
        .split("\n\n")
        .map(parse_grid)
        .collect::<Result<Vec<_>, PuzzleErr>>()
}

fn hflip<T: Clone>(a: &Array2<T>) -> Array2<T> {
    Array2::from_shape_vec(
        a.raw_dim(),
        a.axis_iter(Axis(0)).rev().map(|a| a.to_vec()).concat(),
    )
    .unwrap()
}

fn is_mirror_around(grid: &Array2<bool>, r: usize, with_smudge: bool) -> bool {
    let max_a1 = r + 1;
    let max_a2 = (grid.nrows() - 1) - r;
    let m = cmp::min(max_a1, max_a2);
    let a1 = grid.slice(s![(r + 1 - m)..(r + 1), ..]);
    let a2 = grid.slice(s![(r + 1)..(r + 1 + m), ..]).to_owned();
    if with_smudge {
        Zip::from(&a1)
            .and(&hflip(&a2))
            .map_collect(|x, y| if x == y { 0 } else { 1 })
            .sum()
            == 1
    } else {
        a1 == hflip(&a2)
    }
}

fn find_horizontal_mirror(grid: &Array2<bool>, with_smudge: bool) -> Option<usize> {
    (0..(grid.nrows() - 1)).find(|&r| is_mirror_around(grid, r, with_smudge))
}

fn _solve(input: &str, with_smudge: bool) -> Result<usize, PuzzleErr> {
    let grids = parse_input(input)?;
    let h_results = grids
        .iter()
        .map(|g| find_horizontal_mirror(g, with_smudge))
        .collect::<Vec<_>>();
    Ok(zip(grids.iter(), h_results)
        .map(|(g, h_res)| match h_res {
            Some(x) => (x + 1) * 100,
            None => find_horizontal_mirror(&g.t().to_owned(), with_smudge).unwrap() + 1,
        })
        .sum())
}

pub fn puzzle_1(input: &str) -> Result<usize, PuzzleErr> {
    _solve(input, false)
}

pub fn puzzle_2(input: &str) -> Result<usize, PuzzleErr> {
    _solve(input, true)
}

pub fn main(data_dir: &str) {
    println!("Day 13: Point of Incidence");
    let data = load(data_dir, 13, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(37113));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(30449))
}
