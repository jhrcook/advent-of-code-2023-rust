use crate::data::load;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Failed parsing integer.")]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub fn puzzle_1(input_data: &str) -> Result<u32, PuzzleError> {
    Ok(1)
}

pub fn main(data_dir: &str) {
    println!("Day 1: Calorie Counting");
    let data = load(data_dir, 1, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        _ => panic!("No solution to puzzle 1."),
    }
    assert_eq!(answer_1, Ok(68787));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     _ => panic!("No solution to puzzle 2."),
    // }
    // assert_eq!(answer_2, Ok(198041))
}

#[cfg(test)]
mod tests {
    use crate::solutions::day01::puzzle_1;

    const EXAMPLE_INPUT: &str = "";

    #[test]
    fn example_1_puzzle_1() {
        assert_eq!(puzzle_1(self::EXAMPLE_INPUT), Ok(1));
    }

    // #[test]
    // fn example_1_puzzle_2() {
    //     assert_eq!(puzzle_2(self::EXAMPLE_INPUT), Ok(1));
    // }
}
