use crate::data::load;
use crate::math_utils;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Unknown direction: '{}'.", .0)]
    UnknownDirection(String),
    #[error("Error parsing graph line: '{}'.", .0)]
    GraphLineParsingError(String),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    R,
    L,
}

#[derive(Debug, Clone)]
struct Directions {
    values: Vec<Direction>,
    i: usize,
    len: usize,
}

impl Directions {
    fn new(values: Vec<Direction>) -> Self {
        let len = values.len();
        Self { values, i: 0, len }
    }
}

impl Iterator for Directions {
    type Item = (usize, Direction);
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.len {
            self.i = 0;
        }
        self.i += 1;
        Some((self.i - 1, self.values[self.i - 1]))
    }
}

impl TryFrom<&char> for Direction {
    type Error = PuzzleErr;

    fn try_from(value: &char) -> Result<Self, PuzzleErr> {
        match value {
            'R' => Ok(Direction::R),
            'L' => Ok(Direction::L),
            _ => Err(PuzzleErr::UnknownDirection(value.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
struct Graph {
    id_to_name: HashMap<u32, String>,
    name_to_id: HashMap<String, u32>,
    edges: HashMap<u32, (u32, u32)>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            id_to_name: HashMap::new(),
            name_to_id: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    fn add_node(&mut self, name: &str) -> u32 {
        let name = name.to_string();
        if let Some(id) = self.name_to_id.get(&name) {
            *id
        } else {
            let id = self.id_to_name.len() as u32;
            self.name_to_id.insert(name.clone(), id);
            self.id_to_name.insert(id, name);
            id
        }
    }

    fn get_node_id(&self, name: &str) -> Option<&u32> {
        self.name_to_id.get(name)
    }

    fn add_edge(&mut self, from: &str, to: (&str, &str)) {
        let from_id = self.add_node(from);
        let to_id_1 = self.add_node(to.0);
        let to_id_2 = self.add_node(to.1);
        self.edges.insert(from_id, (to_id_1, to_id_2));
    }

    fn next_step(&self, current_node: &u32, direction: &Direction) -> Option<&u32> {
        match self.edges.get(current_node) {
            Some((l, r)) => match direction {
                Direction::L => Some(l),
                Direction::R => Some(r),
            },
            None => None,
        }
    }
}

lazy_static! {
    static ref GRAPH_LINE_RE: Regex = Regex::new(r"\w+").unwrap();
}

fn _parse_line(line: &str) -> Result<(String, (String, String)), PuzzleErr> {
    let finds = GRAPH_LINE_RE
        .find_iter(line)
        .map(|s| s.as_str().to_string())
        .collect::<Vec<_>>();
    Ok((finds[0].clone(), (finds[1].clone(), finds[2].clone())))
}

fn parse_input(input: &str) -> Result<(Directions, Graph), PuzzleErr> {
    let lines = input.trim().lines().collect::<Vec<_>>();

    // Directions.
    let directions = lines
        .first()
        .unwrap()
        .chars()
        .map(|c| Direction::try_from(&c))
        .collect::<Result<Vec<_>, PuzzleErr>>()?;

    // Graph.
    let mut graph = Graph::new();
    for line in lines.iter().skip(2) {
        let (from, (to_l, to_r)) = _parse_line(line)?;
        graph.add_edge(&from, (&to_l, &to_r));
    }
    Ok((Directions::new(directions), graph))
}

pub fn puzzle_1(input: &str) -> Result<u32, PuzzleErr> {
    let (directions, graph) = parse_input(input)?;
    let mut node = graph.get_node_id("AAA").unwrap();
    let zzz = graph.get_node_id("ZZZ").unwrap();
    let mut count = 0;
    for (_, d) in directions {
        count += 1;
        node = graph.next_step(node, &d).unwrap();
        if node == zzz {
            break;
        }
    }
    Ok(count)
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct StateMap {
    visit_state: (u32, usize),
    start: u32,
    loop_size: u32,
}

fn make_state_map(
    start: &u32,
    end_nodes: &HashSet<&u32>,
    graph: &Graph,
    directions: Directions,
) -> StateMap {
    let mut node = start;
    let mut count = 0;
    let mut state_map = HashMap::new();
    let mut visit_state: Option<(u32, usize)> = None;
    for (i, d) in directions {
        count += 1;
        node = graph.next_step(node, &d).unwrap();
        if let Entry::Vacant(e) = state_map.entry((*node, i)) {
            if end_nodes.contains(node) {
                e.insert(count);
            }
        } else {
            visit_state = Some((*node, i));
            break;
        }
    }
    let start = *state_map.get(&visit_state.unwrap()).unwrap();
    let loop_size = count - start;
    StateMap {
        visit_state: visit_state.unwrap(),
        start,
        loop_size,
    }
}

pub fn puzzle_2(input: &str) -> Result<u64, PuzzleErr> {
    let (directions, graph) = parse_input(input)?;

    let start_nodes = graph
        .id_to_name
        .iter()
        .filter(|(_, n)| n.ends_with('A'))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let end_nodes = graph
        .id_to_name
        .iter()
        .filter(|(_, n)| n.ends_with('Z'))
        .map(|(i, _)| i)
        .collect::<HashSet<_>>();

    let state_maps = start_nodes
        .iter()
        .map(|n| make_state_map(n, &end_nodes, &graph, directions.clone()))
        .collect::<Vec<_>>();

    Ok(math_utils::lcm(
        state_maps.iter().map(|sm| sm.loop_size as u64).collect(),
    ))
}

pub fn main(data_dir: &str) {
    println!("Day 8: Haunted Wasteland");
    let data = load(data_dir, 8, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(19631));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(21003205388413))
}
