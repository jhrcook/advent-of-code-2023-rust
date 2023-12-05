use aoc_2023::solutions::day05::puzzle_1;

const EXAMPLE_INPUT: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

#[test]
fn example_1_puzzle_1() {
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT), Ok(35));
}

// #[test]
// fn example_1_puzzle_2() {
//     assert_eq!(puzzle_2(self::EXAMPLE_INPUT), Ok(30));
// }
