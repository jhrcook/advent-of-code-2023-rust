use crate::data::load;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]

pub enum PuzzleErr {
    #[error("Integer parsing error: '{}'.", .0)]
    IntParseError(String),
}

#[derive(Debug, Clone, Copy)]
struct MapRange {
    source_start: u32,
    dest_start: u32,
    len: u32,
}

impl MapRange {
    fn new(source_start: u32, dest_start: u32, len: u32) -> Self {
        Self {
            source_start,
            dest_start,
            len,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    // source_cat: String,
    // dest_cat: String,
    ranges: Vec<MapRange>,
}

fn line_to_range(line: &str) -> MapRange {
    let vals = line
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    MapRange::new(vals[1], vals[0], vals[2])
}

impl Map {
    fn translate(&self, source_val: &u32) -> u32 {
        for r in self.ranges.iter() {
            let range = r.source_start..(r.source_start + r.len);
            if range.contains(source_val) {
                return source_val - r.source_start + r.dest_start;
            }
        }
        *source_val
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl Almanac {
    fn apply_maps(&self) -> Vec<u32> {
        self.seeds
            .iter()
            .map(|s| {
                let mut res = *s;
                for map in self.maps.iter() {
                    res = map.translate(&res);
                }
                res
            })
            .collect()
    }
}

fn get_seeds(input: &str) -> Result<Vec<u32>, PuzzleErr> {
    input
        .trim()
        .to_string()
        .lines()
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .trim()
        .split("seeds: ")
        .collect::<Vec<_>>()[1]
        .split_whitespace()
        .map(|s| {
            s.parse::<u32>()
                .or(Err(PuzzleErr::IntParseError(s.to_string())))
        })
        .collect::<Result<Vec<_>, PuzzleErr>>()
}

fn get_maps(input: &str) -> Result<Vec<Map>, PuzzleErr> {
    let mut maps = Vec::new();
    let mut map_ranges = Vec::new();
    for line in input.trim().lines().skip(2).map(|l| l.trim()) {
        if line.contains("map:") {
            continue;
        } else if line.is_empty() {
            maps.push(Map {
                ranges: map_ranges.clone(),
            });
            map_ranges.clear();
        } else {
            map_ranges.push(line_to_range(line));
        }
    }

    if !map_ranges.is_empty() {
        maps.push(Map {
            ranges: map_ranges.clone(),
        })
    }
    Ok(maps)
}

fn parse_input(input: &str) -> Result<Almanac, PuzzleErr> {
    let seeds = get_seeds(input)?; //[1..2].to_vec();
    let maps = get_maps(input)?;
    Ok(Almanac { seeds, maps })
}

pub fn puzzle_1(input: &str) -> Result<u32, PuzzleErr> {
    let almanac = parse_input(input)?;
    let results = almanac.apply_maps();
    Ok(*results.iter().min().unwrap())
}

pub fn main(data_dir: &str) {
    println!("Day 5: If You Give A Seed A Fertilizer");
    let data = load(data_dir, 5, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(650599855));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(6050769))
}
