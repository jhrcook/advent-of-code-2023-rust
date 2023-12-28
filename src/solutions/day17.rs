use crate::data::load;
use num::Complex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Integer parsing error.")]
    ParseIntError(#[from] std::num::ParseIntError),
}

fn parse_grid(input: &str) -> Result<HashMap<Complex<i32>, i32>, PuzzleErr> {
    let mut grid = HashMap::new();
    for (i, r) in input.trim().lines().enumerate() {
        for (j, c) in r.trim().chars().enumerate() {
            grid.insert(
                Complex::<i32>::new(i as i32, j as i32),
                c.to_string().parse::<i32>()?,
            );
        }
    }
    Ok(grid)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    coord: Complex<i32>,
    prev_dir: Complex<i32>,
    score: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.coord.re.cmp(&other.coord.re))
            .then_with(|| self.coord.im.cmp(&other.coord.im))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn turns(d: Complex<i32>) -> Vec<Complex<i32>> {
    Vec::from_iter([Complex { re: 0, im: 1 } / d, Complex { re: 0, im: -1 } / d])
}

fn shortest_path(
    start: &Complex<i32>,
    end: &Complex<i32>,
    grid: &HashMap<Complex<i32>, i32>,
    min_steps: i32,
    max_steps: i32,
) -> i32 {
    let mut queue = BinaryHeap::<State>::from_iter([
        State {
            coord: *start,
            prev_dir: Complex { re: 1, im: 0 },
            score: 0,
        },
        State {
            coord: *start,
            prev_dir: Complex { re: 0, im: 1 },
            score: 0,
        },
    ]);
    let mut visited = HashSet::<(Complex<i32>, Complex<i32>)>::new();
    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        if &state.coord == end {
            return state.score;
        }
        if visited.contains(&(state.coord, state.prev_dir)) {
            continue;
        }
        visited.insert((state.coord, state.prev_dir));

        for next_dir in turns(state.prev_dir) {
            for i in min_steps..=max_steps {
                if grid.contains_key(&(state.coord + (next_dir * i))) {
                    let y: i32 = (1..=i)
                        .map(|j| grid.get(&(state.coord + next_dir * j)).unwrap())
                        .sum();
                    let new_state = State {
                        coord: state.coord + next_dir * i,
                        prev_dir: next_dir,
                        score: state.score + y,
                    };
                    queue.push(new_state);
                }
            }
        }
    }
    unreachable!();
}

pub fn puzzle_1(input: &str) -> Result<i32, PuzzleErr> {
    let grid = parse_grid(input)?;
    let start = Complex { re: 0, im: 0 };
    let end_re = grid.keys().map(|x| x.re).max().unwrap();
    let end_im = grid.keys().map(|x| x.im).max().unwrap();
    let end = Complex {
        re: end_re,
        im: end_im,
    };
    Ok(shortest_path(&start, &end, &grid, 1, 3))
}

pub fn puzzle_2(input: &str) -> Result<i32, PuzzleErr> {
    let grid = parse_grid(input)?;
    let start = Complex { re: 0, im: 0 };
    let end_re = grid.keys().map(|x| x.re).max().unwrap();
    let end_im = grid.keys().map(|x| x.im).max().unwrap();
    let end = Complex {
        re: end_re,
        im: end_im,
    };
    Ok(shortest_path(&start, &end, &grid, 4, 10))
}

pub fn main(data_dir: &str) {
    println!("Day 17: Clumsy Crucible");
    let data = load(data_dir, 17, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(1076));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(1219))
}
