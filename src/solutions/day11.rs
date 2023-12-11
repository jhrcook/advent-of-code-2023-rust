use crate::data::load;
use ndarray::prelude::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Unknown pipe character: {}.", .0)]
    UnknownPipeChar(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    r: usize,
    c: usize,
}

#[derive(Debug, Clone)]
struct CosmicMap {
    arr: Array2<u8>,
    galaxies: Vec<Coord>,
}

fn parse_map(input: &str) -> Result<CosmicMap, PuzzleErr> {
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
    Ok(CosmicMap { arr, galaxies })
}

fn _expand_arr_rows(arr: Array2<u8>) -> Array2<u8> {
    let mut vecs = Vec::new();

    let mut nrows = 0;
    for r in arr.rows() {
        nrows += 1;
        vecs.extend_from_slice(r.to_vec().as_slice());
        if r.sum() == 0 {
            nrows += 1;
            vecs.extend_from_slice(r.to_vec().as_slice());
        }
    }
    Array2::from_shape_vec((nrows, arr.ncols()), vecs).unwrap()
}

fn _locate_galaxies(arr: &Array2<u8>) -> Vec<Coord> {
    let mut galaxies = Vec::new();
    for i in 0..arr.nrows() {
        for j in 0..arr.ncols() {
            if arr.get((i, j)).unwrap() == &1 {
                galaxies.push(Coord { r: i, c: j })
            }
        }
    }
    galaxies
}

fn expand_map(map: CosmicMap) -> CosmicMap {
    let arr = _expand_arr_rows(map.arr);
    let arr = _expand_arr_rows(arr.t().to_owned()).t().to_owned();
    let galaxies = _locate_galaxies(&arr);
    CosmicMap { arr, galaxies }
}

fn calculate_dists(map: &CosmicMap) -> Vec<usize> {
    let mut dists = Vec::new();
    for (i, a) in map.galaxies.iter().enumerate() {
        for b in map.galaxies[(i + 1)..].iter() {
            dists.push(a.r.abs_diff(b.r) + a.c.abs_diff(b.c))
        }
    }
    dists
}

pub fn puzzle_1(input: &str) -> Result<usize, PuzzleErr> {
    let map = parse_map(input)?;
    log::info!("STARTING MAP:\n{:?}", map.arr);
    log::info!("Galaxy locations: {:?}", map.galaxies);

    let map = expand_map(map);
    log::info!("EXPANDED MAP:\n{:?}", map.arr);
    log::info!("Galaxy locations: {:?}", map.galaxies);

    Ok(calculate_dists(&map).iter().sum())
}

pub fn main(data_dir: &str) {
    println!("Day 11: Cosmic Expansion");
    let data = load(data_dir, 11, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(9724940));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(933))
}
