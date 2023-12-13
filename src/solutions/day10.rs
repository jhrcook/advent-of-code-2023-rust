use crate::data::load;
use petgraph::algo::{self, DfsSpace};
use petgraph::{graph::NodeIndex, graph::UnGraph};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Unknown pipe character: {}.", .0)]
    UnknownPipeChar(String),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

impl Pipe {
    fn is_turn(&self) -> bool {
        matches!(self, Pipe::NE | Pipe::NW | Pipe::SW | Pipe::SE)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    r: i32,
    c: i32,
}

#[derive(Debug, Clone)]
struct Maze {
    map: HashMap<Coord, Pipe>,
    graph: UnGraph<Coord, ()>,
    start_i: NodeIndex,
    node_indices: HashMap<Coord, NodeIndex>,
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
    nodes: &HashMap<Coord, NodeIndex>,
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

fn _map_to_graph(
    map: &HashMap<Coord, Pipe>,
) -> (UnGraph<Coord, ()>, HashMap<Coord, NodeIndex>, NodeIndex) {
    let mut nodes = HashMap::new();
    let mut graph = UnGraph::new_undirected();
    let mut start_i: Option<NodeIndex> = None;
    for (c, p) in map.iter() {
        match p {
            Pipe::G => (),
            Pipe::S => {
                start_i = Some(graph.add_node(*c));
                nodes.insert(*c, start_i.unwrap());
            }
            _ => {
                nodes.insert(*c, graph.add_node(*c));
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

    // Only retain nodes connected to S.
    let mut dfs_space = DfsSpace::default();
    let keep_nodes = graph
        .node_indices()
        .filter(|n| algo::has_path_connecting(&graph, start_i.unwrap(), *n, Some(&mut dfs_space)))
        .collect::<HashSet<_>>();
    graph.retain_nodes(|_, n| keep_nodes.contains(&n));

    (graph, nodes, start_i.unwrap())
}

fn parse_input(data: &str) -> Result<Maze, PuzzleErr> {
    let map = _build_map(data)?;
    let (graph, node_indices, start_i) = _map_to_graph(&map);
    Ok(Maze {
        map,
        graph,
        start_i,
        node_indices,
    })
}

pub fn puzzle_1(input: &str) -> Result<i32, PuzzleErr> {
    let maze = parse_input(input)?;
    Ok(*algo::dijkstra(&maze.graph, maze.start_i, None, |_| 1)
        .values()
        .max()
        .unwrap())
}

fn _counter_turning_pipes(a: &Pipe, b: &Pipe) -> bool {
    matches!(
        (a, b),
        (Pipe::NW, Pipe::SE) | (Pipe::SE, Pipe::NW) | (Pipe::SW, Pipe::NE) | (Pipe::NE, Pipe::SW)
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Decision {
    Break,
    Continue,
    Insert,
    UpdatePrevTurn,
}

fn collection_decision(
    coord: &Coord,
    pipe: &Pipe,
    graph_nodes: &HashMap<Coord, NodeIndex>,
    prev_turn: &Option<Pipe>,
    straight_breaking_pipe: &Pipe,
) -> Decision {
    if !graph_nodes.contains_key(coord) {
        log::debug!("Insert coord.");
        return Decision::Insert;
    }

    if pipe == straight_breaking_pipe {
        log::debug!("Break pipe.");
        return Decision::Break;
    }

    if !pipe.is_turn() {
        log::debug!("Straight pipe, just continue.");
        return Decision::Continue;
    }

    log::debug!("Previous, current pipes:  {:?}, {:?}", prev_turn, pipe);
    match (prev_turn, pipe) {
        (None, _) => {
            log::debug!("Update previous turning pipe.");
            Decision::UpdatePrevTurn
        }
        (Some(a), b) => {
            if _counter_turning_pipes(a, b) {
                log::debug!("Counter turning pipes, break.");
                Decision::Break
            } else {
                log::debug!("Not-counter turning pipes, continue.");
                Decision::UpdatePrevTurn
            }
        }
    }
}

fn get_maxes<T>(map: &HashMap<Coord, T>) -> (i32, i32) {
    let max_r = map.keys().map(|c| c.r).max().unwrap();
    let max_c = map.keys().map(|c| c.c).max().unwrap();
    (max_r, max_c)
}

fn collect_nodes_visible_from_outside(
    map: &HashMap<Coord, Pipe>,
    graph_nodes: &HashMap<Coord, NodeIndex>,
) -> HashSet<Coord> {
    let mut outsides = HashSet::new();
    let (max_r, max_c) = get_maxes(map);
    for r in 0..=max_r {
        let mut prev_turn: Option<Pipe> = None;
        for c in 0..=max_c {
            let coord = Coord { r, c };
            let pipe = map.get(&coord).unwrap();
            log::debug!("Checking coord {:?} with character {:?}", coord, pipe);
            match collection_decision(&coord, pipe, graph_nodes, &prev_turn, &Pipe::V) {
                Decision::Continue => {}
                Decision::Break => break,
                Decision::Insert => {
                    outsides.insert(coord);
                }
                Decision::UpdatePrevTurn => {
                    prev_turn = Some(*pipe);
                }
            }
        }

        prev_turn = None;
        for c in (0..=max_c).rev() {
            let coord = Coord { r, c };
            let pipe = map.get(&coord).unwrap();
            match collection_decision(&coord, pipe, graph_nodes, &prev_turn, &Pipe::V) {
                Decision::Continue => {}
                Decision::Break => break,
                Decision::Insert => {
                    outsides.insert(coord);
                }
                Decision::UpdatePrevTurn => {
                    prev_turn = Some(*pipe);
                }
            }
        }
    }

    for c in 0..=max_c {
        let mut prev_turn: Option<Pipe> = None;
        for r in 0..=max_r {
            let coord = Coord { r, c };
            let pipe = map.get(&coord).unwrap();
            match collection_decision(&coord, pipe, graph_nodes, &prev_turn, &Pipe::H) {
                Decision::Continue => {}
                Decision::Break => break,
                Decision::Insert => {
                    outsides.insert(coord);
                }
                Decision::UpdatePrevTurn => {
                    prev_turn = Some(*pipe);
                }
            }
        }

        prev_turn = None;
        for r in (0..=max_r).rev() {
            let coord = Coord { r, c };
            let pipe = map.get(&coord).unwrap();
            match collection_decision(&coord, pipe, graph_nodes, &prev_turn, &Pipe::H) {
                Decision::Continue => {}
                Decision::Break => break,
                Decision::Insert => {
                    outsides.insert(coord);
                }
                Decision::UpdatePrevTurn => {
                    prev_turn = Some(*pipe);
                }
            }
        }
    }

    outsides
}

fn make_opposite_graphs(
    input_map: &HashMap<Coord, Pipe>,
    graph1_nodes: &HashMap<Coord, NodeIndex>,
) -> (UnGraph<Coord, ()>, HashMap<Coord, NodeIndex>) {
    let mut outs_graph = UnGraph::new_undirected();
    let outs_nodes = input_map
        .iter()
        .filter(|(c, _)| !graph1_nodes.contains_key(c))
        .map(|(c, _)| (*c, outs_graph.add_node(*c)))
        .collect::<HashMap<Coord, NodeIndex>>();

    let (max_r, max_c) = get_maxes(&outs_nodes);

    for (coord, idx) in outs_nodes.iter() {
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let neighbor_coord = Coord {
                r: coord.r + dr,
                c: coord.c + dc,
            };
            if (neighbor_coord.r < 0)
                | (neighbor_coord.c < 0)
                | (neighbor_coord.r > max_r)
                | (neighbor_coord.c > max_c)
            {
                continue;
            } else if !graph1_nodes.contains_key(&neighbor_coord) {
                log::debug!("Making edge: {:?}  -->  {:?}", coord, neighbor_coord);
                let neighbor_idx = outs_nodes.get(&neighbor_coord).unwrap();
                if !outs_graph.contains_edge(*idx, *neighbor_idx) {
                    outs_graph.add_edge(*idx, *neighbor_idx, ());
                }
            }
        }
    }
    (outs_graph, outs_nodes)
}

fn log_map(map: &HashMap<Coord, Pipe>, nodes: &HashMap<Coord, NodeIndex>) {
    let (max_r, max_c) = get_maxes(map);
    let mut row_strs = Vec::new();
    for r in 0..=max_r {
        let mut row = Vec::new();
        for c in 0..=max_c {
            let coord = Coord { r, c };
            if nodes.contains_key(&coord) {
                row.push(".")
            } else {
                row.push(" ")
            }
        }
        row_strs.push(row.join(" ").to_string())
    }
    let graph_str = row_strs.join("\n");
    log::info!("MAZE:\n{}", graph_str);
}

// !NOTE: Currently fails the fourth example.
// !Seems like there is a bug in the building of the starting map.
// (Answer with current algorithm, 404, is too low.)

pub fn puzzle_2(input: &str) -> Result<usize, PuzzleErr> {
    // 1. Create graph from puzzle 1.
    let maze = parse_input(input)?;
    log_map(&maze.map, &maze.node_indices);

    // 2. Create a graph all other nodes not in graph 1.
    let (mut outside_graph, node_idx) = make_opposite_graphs(&maze.map, &maze.node_indices);
    // use petgraph::dot::{Config, Dot};
    // log::debug!(
    //     "\n{:?}",
    //     Dot::with_config(&outside_graph, &[Config::EdgeNoLabel])
    // );

    // 3. Collect nodes directly "visible" from the outside.
    let outsides = collect_nodes_visible_from_outside(&maze.map, &maze.node_indices);
    log::debug!("Outsides:");
    for o in outsides.iter() {
        log::debug!(" {:?}", o);
    }

    // 4. Remove nodes connected to the nodes "visible" from the outside.
    let mut outsides_idx = HashSet::new();
    for out in outsides.iter() {
        outsides_idx.insert(node_idx.get(out).unwrap());
    }
    let mut dfs_space = DfsSpace::default();
    for n_i in node_idx.values() {
        for out in outsides.iter() {
            let out_i = *node_idx.get(out).unwrap();
            if algo::has_path_connecting(&outside_graph, *n_i, out_i, Some(&mut dfs_space)) {
                outsides_idx.insert(n_i);
                break;
            }
        }
    }
    outside_graph.retain_nodes(|_, n| !outsides_idx.contains(&n));

    // Solution is the number of remaining nodes.
    Ok(outside_graph.node_count())
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
    // 404 (too low)
    // assert_eq!(answer_2, Ok(933))
}
