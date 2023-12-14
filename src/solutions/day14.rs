use crate::data::load;
use cached::proc_macro::cached;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Input parsing error.")]
    ParseInputError,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    r: isize,
    c: isize,
}

impl Coord {
    fn new(r: isize, c: isize) -> Self {
        Coord { r, c }
    }
}

impl Coord {
    fn rot90(&self, r_max: &isize) -> Coord {
        Coord {
            r: self.c,
            c: r_max - self.r,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Rocks {
    round: HashSet<Coord>,
    square: HashSet<Coord>,
    height: isize,
    width: isize,
}

impl Hash for Rocks {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut v_round = self.round.iter().collect::<Vec<_>>();
        v_round.sort();
        v_round.hash(state);
        let mut v_square = self.square.iter().collect::<Vec<_>>();
        v_square.sort();
        v_square.hash(state);
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

#[cached]
fn tilt_north(rocks: Rocks) -> Rocks {
    let mut new_rocks = rocks.clone();
    for r in 0..rocks.height {
        for c in 0..rocks.width {
            let coord = Coord::new(r, c);
            if new_rocks.round.contains(&coord) {
                new_rocks.round.remove(&coord);
                new_rocks
                    .round
                    .insert(find_north_position(&coord, &new_rocks));
            }
        }
    }
    new_rocks
}

fn calc_total_load(rocks: &Rocks) -> isize {
    rocks.round.iter().map(|coord| rocks.height - coord.r).sum()
}

pub fn puzzle_1(input: &str) -> isize {
    let mut rocks = parse_input(input);
    rocks = tilt_north(rocks);
    calc_total_load(&rocks)
}

#[cached]
fn rotate_rocks(rocks: Rocks) -> Rocks {
    let mut new_rocks = rocks.clone();
    new_rocks.round = new_rocks
        .round
        .iter()
        .map(|c| c.rot90(&(new_rocks.height - 1)))
        .collect();
    new_rocks.square = new_rocks
        .square
        .iter()
        .map(|c| c.rot90(&(new_rocks.height - 1)))
        .collect();
    (new_rocks.height, new_rocks.width) = (new_rocks.width, new_rocks.height);
    new_rocks
}

#[cached]
fn rotation_cycle(mut rocks: Rocks) -> Rocks {
    for _ in 0..4 {
        rocks = rotate_rocks(tilt_north(rocks));
    }
    rocks
}

pub fn puzzle_2(input: &str, n_cycles: usize) -> isize {
    let mut rocks = parse_input(input);

    let mut prev_cycle_cache: HashMap<Rocks, Vec<usize>> = HashMap::new();
    let mut i = 0;
    while i != n_cycles {
        rocks = rotation_cycle(rocks.clone());
        i += 1;

        let prev_i = prev_cycle_cache.entry(rocks.clone()).or_default();

        if let Some(new_i) = prev_i
            .iter()
            .map(|j| i + (i - j))
            .filter(|new_i| new_i <= &n_cycles)
            .max()
        {
            i = new_i;
        }
        prev_i.push(i);
    }

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
    let answer_2 = puzzle_2(&data, 1000000000);
    println!(" Puzzle 2: {}", answer_2);
    assert_eq!(answer_2, 104619);
}
// 104639 (too high)
