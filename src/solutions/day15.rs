use crate::data::load;

fn ascii_hash(s: &str) -> usize {
    s.trim()
        .chars()
        .map(|c| c as u8 as usize)
        .fold(0, |a, b| ((a + b) * 17) % 256)
}

pub fn puzzle_1(input: &str) -> usize {
    input.split(',').map(ascii_hash).sum()
}

#[derive(Debug, Clone, Copy)]
struct Lens<'a> {
    label: &'a str,
    focal_len: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    D,
    E,
}

impl From<&char> for Operation {
    fn from(value: &char) -> Self {
        match value {
            '-' => Operation::D,
            '=' => Operation::E,
            _ => panic!("Unkown op: {}", value),
        }
    }
}

fn calc_focussing_power(boxes: &[Vec<Lens>]) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(b_idx, b)| {
            b.iter()
                .enumerate()
                .map(|(lens_idx, lens)| (b_idx + 1) * (lens_idx + 1) * lens.focal_len.unwrap())
                .sum::<usize>()
        })
        .sum()
}

fn parse_step(step: &str) -> (Lens, Operation) {
    if step.contains('-') {
        return (
            Lens {
                label: step.split('-').collect::<Vec<_>>()[0],
                focal_len: None,
            },
            Operation::D,
        );
    }
    let split = step.split('=').collect::<Vec<_>>();
    let label = split[0];
    let focal_len = Some(split[1].parse::<usize>().unwrap());
    (Lens { label, focal_len }, Operation::E)
}

pub fn puzzle_2(input: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = (0..256).map(|_| Vec::new()).collect::<Vec<_>>();
    for step in input.trim().split(',') {
        let (lens, op) = parse_step(step);
        let box_i = ascii_hash(lens.label);
        let _box = boxes.get_mut(box_i).unwrap();
        match op {
            Operation::D => {
                _box.retain(|l| l.label != lens.label);
            }
            Operation::E => {
                if let Some(lens_i) = _box.iter().position(|l| l.label == lens.label) {
                    let _lens = _box.get_mut(lens_i).unwrap();
                    _lens.focal_len = lens.focal_len;
                } else {
                    _box.push(lens);
                }
            }
        }
    }
    calc_focussing_power(&boxes)
}

pub fn main(data_dir: &str) {
    println!("Day 15: Lens Library");
    let data = load(data_dir, 15, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    println!(" Puzzle 1: {}", answer_1);
    assert_eq!(answer_1, 504036);

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    println!(" Puzzle 2: {}", answer_2);
    // assert_eq!(answer_2, Ok(30449))
}
