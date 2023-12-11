use crate::data::load;
use ndarray::prelude::*;
use std::iter::zip;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    r: usize,
    c: usize,
}

impl Coord {
    fn t(&self) -> Self {
        Self {
            r: self.c,
            c: self.r,
        }
    }

    fn dist(&self, other: &Self) -> usize {
        self.r.abs_diff(other.r) + self.c.abs_diff(other.c)
    }
}

#[derive(Debug, Clone)]
struct CosmicMap {
    arr: Array2<u8>,
    galaxies: Vec<Coord>,
}

impl CosmicMap {
    fn t(&self) -> Self {
        Self {
            arr: self.arr.t().to_owned(),
            galaxies: self.galaxies.iter().map(|c| c.t()).collect(),
        }
    }
}

fn parse_map(input: &str) -> CosmicMap {
    let lines = input.trim().lines().map(|l| l.trim()).collect::<Vec<_>>();
    let width = lines.first().unwrap().len();
    let height = lines.len();
    let mut arr = Array2::zeros((height, width));
    let mut galaxies = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                arr.slice_mut(s![i, j]).fill(1);
                galaxies.push(Coord { r: i, c: j });
            }
        }
    }
    CosmicMap { arr, galaxies }
}

fn calculate_dists(coords: &[Coord]) -> Vec<usize> {
    let mut dists = Vec::new();
    for (i, a) in coords.iter().enumerate() {
        for b in coords[(i + 1)..].iter() {
            dists.push(a.dist(b))
        }
    }
    dists
}

fn expand_galaxies(map: &CosmicMap, ex_rate: usize) -> Vec<Coord> {
    let x = match ex_rate {
        0..=1 => ex_rate,
        _ => ex_rate - 1,
    };
    let coords = map.galaxies.clone();
    let mut add_rs = (0..coords.len()).map(|_| 0).collect::<Vec<_>>();

    for i in 0..map.arr.nrows() {
        if map.arr.row(i).sum() == 0 {
            add_rs = zip(&coords, add_rs)
                .map(|(coord, add_r)| if coord.r > i { add_r + x } else { add_r })
                .collect();
        }
    }

    zip(coords, add_rs)
        .map(|(coord, add_r)| Coord {
            r: coord.r + add_r,
            c: coord.c,
        })
        .collect()
}

fn _solve(input: &str, expansion_size: usize) -> usize {
    let mut map = parse_map(input);
    map.galaxies = expand_galaxies(&map, expansion_size);
    map = map.t();
    map.galaxies = expand_galaxies(&map, expansion_size);
    calculate_dists(&map.galaxies).iter().sum()
}

pub fn puzzle_1(input: &str) -> usize {
    _solve(input, 1)
}

pub fn puzzle_2(input: &str, expansion_size: usize) -> usize {
    _solve(input, expansion_size)
}

pub fn main(data_dir: &str) {
    println!("Day 11: Cosmic Expansion");
    let data = load(data_dir, 11, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    println!(" Puzzle 1: {}", answer_1);
    assert_eq!(answer_1, 9724940);

    // Puzzle 2.
    let answer_2 = puzzle_2(&data, 1000000);
    println!(" Puzzle 1: {}", answer_2);
    assert_eq!(answer_2, 569052586852);
}
