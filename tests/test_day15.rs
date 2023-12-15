use aoc_2023::solutions::day15::{puzzle_1, puzzle_2};

const EXAMPLE_INPUT_1: &str = "HASH";
const EXAMPLE_INPUT_2: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[test]
fn puzzle_1_example_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_1), 52);
}
#[test]
fn puzzle_1_example_2() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_2), 1320);
}

#[test]
fn puzzle_2_example_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_2), 145);
}
