use crate::data::load;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Brick {
    a: Pos,
    b: Pos,
}

fn parse_line(line: &str) -> Brick {
    let poss: Vec<Pos> = line
        .split('~')
        .map(|s| {
            let ints: Vec<u32> = s.split(',').map(|a| a.parse::<u32>().unwrap()).collect();
            Pos {
                x: ints[0],
                y: ints[1],
                z: ints[2],
            }
        })
        .collect();
    Brick {
        a: poss[0],
        b: poss[1],
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    input.trim().lines().map(parse_line).collect()
}

pub fn puzzle_1(input: &str) -> u32 {
    let bricks = parse_input(input);
    bricks.iter().for_each(|b| println!("{:?}", b));
    0
}

pub fn main(data_dir: &str) {
    println!("Day 22: Sand Slabs");
    let data = load(data_dir, 22, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    println!(" Puzzle 1: {}", answer_1)

    // assert_eq!(answer_1, Ok(37113));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("No solution to puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(30449))
}
