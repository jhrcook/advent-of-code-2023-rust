use crate::data::load;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Input parsing error: {}.", .0)]
    ParseInputError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaveObject {
    Empty,     // "."
    MirrorL,   // "\"
    MirrorR,   // "/"
    SplitterV, // "|"
    SplitterH, // "-"
}

impl TryFrom<&char> for CaveObject {
    type Error = PuzzleErr;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(CaveObject::Empty),
            '\\' => Ok(CaveObject::MirrorL),
            '/' => Ok(CaveObject::MirrorR),
            '|' => Ok(CaveObject::SplitterV),
            '-' => Ok(CaveObject::SplitterH),
            _ => Err(PuzzleErr::ParseInputError(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    r: i32,
    c: i32,
}

fn parse_input(input: &str) -> Result<HashMap<Coord, CaveObject>, PuzzleErr> {
    let mut grid = HashMap::new();
    for (r, line) in input.trim().lines().enumerate() {
        for (c, item) in line.trim().chars().enumerate() {
            let coord = Coord {
                r: r as i32,
                c: c as i32,
            };
            let obj = CaveObject::try_from(&item)?;
            grid.insert(coord, obj);
        }
    }
    Ok(grid)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    loc: Coord,
    dir: Direction,
}

impl Beam {
    fn new(loc: Coord, dir: Direction) -> Self {
        Beam { loc, dir }
    }

    fn move_up(&self) -> Self {
        let mut b = *self;
        b.dir = Direction::Up;
        b.loc.r -= 1;
        b
    }
    fn move_down(&self) -> Self {
        let mut b = *self;
        b.dir = Direction::Down;
        b.loc.r += 1;
        b
    }
    fn move_left(&self) -> Self {
        let mut b = *self;
        b.dir = Direction::Left;
        b.loc.c -= 1;
        b
    }
    fn move_right(&self) -> Self {
        let mut b = *self;
        b.dir = Direction::Right;
        b.loc.c += 1;
        b
    }
}

fn decide_next_beam(beam: &Beam, cave_obj: &CaveObject) -> Vec<Beam> {
    match cave_obj {
        CaveObject::Empty => match beam.dir {
            Direction::Up => [beam.move_up()].to_vec(),
            Direction::Down => [beam.move_down()].to_vec(),
            Direction::Left => [beam.move_left()].to_vec(),
            Direction::Right => [beam.move_right()].to_vec(),
        },
        CaveObject::MirrorL => match beam.dir {
            Direction::Up => [beam.move_left()].to_vec(),
            Direction::Down => [beam.move_right()].to_vec(),
            Direction::Left => [beam.move_up()].to_vec(),
            Direction::Right => [beam.move_down()].to_vec(),
        },
        CaveObject::MirrorR => match beam.dir {
            Direction::Up => [beam.move_right()].to_vec(),
            Direction::Down => [beam.move_left()].to_vec(),
            Direction::Left => [beam.move_down()].to_vec(),
            Direction::Right => [beam.move_up()].to_vec(),
        },
        CaveObject::SplitterV => match beam.dir {
            Direction::Up => [beam.move_up()].to_vec(),
            Direction::Down => [beam.move_down()].to_vec(),
            Direction::Left | Direction::Right => [beam.move_up(), beam.move_down()].to_vec(),
        },
        CaveObject::SplitterH => match beam.dir {
            Direction::Up | Direction::Down => [beam.move_left(), beam.move_right()].to_vec(),
            Direction::Left => [beam.move_left()].to_vec(),
            Direction::Right => [beam.move_right()].to_vec(),
        },
    }
}

fn move_beam(beam: &Beam, grid: &HashMap<Coord, CaveObject>, beam_tracker: &mut HashSet<Beam>) {
    if beam_tracker.contains(beam) {
        return;
    }
    let Some(cave_obj) = grid.get(&beam.loc) else {
        return;
    };
    beam_tracker.insert(*beam);
    for next_beam in decide_next_beam(beam, cave_obj) {
        move_beam(&next_beam, grid, beam_tracker);
    }
}

pub fn puzzle_1(input: &str) -> Result<usize, PuzzleErr> {
    let grid = parse_input(input)?;
    let mut energized_coords = HashSet::new();
    let beam = Beam::new(Coord { r: 0, c: 0 }, Direction::Right);
    move_beam(&beam, &grid, &mut energized_coords);
    Ok(energized_coords
        .iter()
        .map(|b| b.loc)
        .collect::<HashSet<_>>()
        .len())
}

pub fn main(data_dir: &str) {
    println!("Day 16: The Floor Will Be Lava");
    let data = load(data_dir, 16, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(6921));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(30449))
}
