use crate::data::load;
use petgraph::algo;
use petgraph::{graph::NodeIndex, graph::UnGraph};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Unknown pipe character: {}.", .0)]
    UnknownPipeChar(String),
}

#[derive(Debug, Clone, Copy, Hash)]
enum Pipe {
    V,
    H,
    NE,
    NW,
    SW,
    SE,
    G,
    S,
}

impl TryFrom<&char> for Pipe {
    type Error = PuzzleErr;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::V),
            '-' => Ok(Pipe::H),
            'L' => Ok(Pipe::NE),
            'J' => Ok(Pipe::NW),
            '7' => Ok(Pipe::SW),
            'F' => Ok(Pipe::SE),
            '.' => Ok(Pipe::G),
            'S' => Ok(Pipe::S),
            c => Err(PuzzleErr::UnknownPipeChar(c.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    r: i32,
    c: i32,
}

#[derive(Debug, Clone)]
struct Maze {
    _map: HashMap<Coord, Pipe>,
    graph: UnGraph<Coord, ()>,
    start_i: NodeIndex,
}

fn _build_map(input: &str) -> Result<HashMap<Coord, Pipe>, PuzzleErr> {
    let mut map = HashMap::new();
    for (r, line) in input.trim().lines().enumerate() {
        for (c, x) in line.trim().chars().enumerate() {
            map.insert(
                Coord {
                    r: r as i32,
                    c: c as i32,
                },
                Pipe::try_from(&x)?,
            );
        }
    }
    Ok(map)
}

fn _add_neighbors(
    graph: &mut UnGraph<Coord, ()>,
    nodes: &HashMap<&Coord, NodeIndex>,
    node_idx: &NodeIndex,
    coord: &Coord,
    directions: &Vec<(i32, i32)>,
) {
    for (dr, dc) in directions {
        let neighbor = Coord {
            r: coord.r + dr,
            c: coord.c + dc,
        };
        if let Some(neighbor_i) = nodes.get(&neighbor) {
            graph.add_edge(*node_idx, *neighbor_i, ());
        }
    }
}

fn _map_to_graph(map: &HashMap<Coord, Pipe>) -> (UnGraph<Coord, ()>, NodeIndex) {
    let mut nodes = HashMap::new();
    let mut graph = UnGraph::new_undirected();
    let mut start_i: Option<NodeIndex> = None;
    for (c, p) in map.iter() {
        match p {
            Pipe::G => (),
            Pipe::S => {
                start_i = Some(graph.add_node(*c));
                nodes.insert(c, start_i.unwrap());
            }
            _ => {
                nodes.insert(c, graph.add_node(*c));
            }
        };
    }

    for (coord, p) in map.iter() {
        if let Some(node_idx) = nodes.get(coord) {
            let directions = match p {
                Pipe::V => Vec::from_iter([(-1, 0), (1, 0)]),
                Pipe::H => Vec::from_iter([(0, -1), (0, 1)]),
                Pipe::NE => Vec::from_iter([(-1, 0), (0, 1)]),
                Pipe::NW => Vec::from_iter([(-1, 0), (0, -1)]),
                Pipe::SW => Vec::from_iter([(1, 0), (0, -1)]),
                Pipe::SE => Vec::from_iter([(1, 0), (0, 1)]),
                Pipe::S => Vec::from_iter([(-1, 0), (1, 0), (0, -1), (0, 1)]),
                Pipe::G => Vec::new(),
            };
            _add_neighbors(&mut graph, &nodes, node_idx, coord, &directions);
        }
    }

    // Only keep edges between nodes with two edges at this point.
    let node_indices = graph.node_indices().collect::<Vec<_>>();
    let mut keep_edges = HashSet::new();
    for (i, n1) in node_indices.iter().enumerate() {
        for n2 in node_indices[(i + 1)..].iter() {
            let edges = graph.edges_connecting(*n1, *n2).collect::<Vec<_>>();
            if edges.len() == 2 {
                keep_edges.insert(graph.find_edge(*n1, *n2).unwrap());
            }
        }
    }
    graph.retain_edges(|_, e| keep_edges.contains(&e));

    (graph, start_i.unwrap())
}

fn parse_input(data: &str) -> Result<Maze, PuzzleErr> {
    let map = _build_map(data)?;
    let (graph, start_i) = _map_to_graph(&map);
    Ok(Maze {
        _map: map,
        graph,
        start_i,
    })
}

pub fn puzzle_1(input: &str) -> Result<i32, PuzzleErr> {
    let maze = parse_input(input)?;
    assert!(algo::is_cyclic_undirected(&maze.graph));

    Ok(*algo::dijkstra(&maze.graph, maze.start_i, None, |_| 1)
        .values()
        .max()
        .unwrap())
}

pub fn main(data_dir: &str) {
    println!("Day 10: Pipe Maze");
    let data = load(data_dir, 10, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(6867));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(933))
}

// 233 too low
// 234 too low
// 399 too low

// TRY: 439
