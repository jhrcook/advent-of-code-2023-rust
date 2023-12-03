use aoc_2023::solutions::day03::puzzle_1;

const EXAMPLE_INPUT: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

#[test]
fn example_1_puzzle_1() {
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT), Ok(4361));
}

const EXAMPLE_INPUT_2: &str = "
............677..........................................................................227.....730..35.......318...........92...166.......
....%..863..#......................36.............956..337%......692..............*744....$..........*......../.....187..-..................
..346...*.....475.440....903&..996*...404+.395...*..............*.......&253.223.....................453..535......@....265.....290$........
";

#[test]
fn example_2_puzzle_1() {
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_2), Ok(8982));
}

// #[test]
// fn example_1_puzzle_2() {
//     assert_eq!(puzzle_2(self::EXAMPLE_INPUT), Ok(2286));
// }
