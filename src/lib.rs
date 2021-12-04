pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub const PUZZLES: [Puzzle; 5] = [
    day1::PUZZLE,
    day2::PUZZLE,
    day3::PUZZLE,
    day4::PUZZLE,
    day5::PUZZLE,
];

pub struct Puzzle {
    pub name: &'static str,
    pub solve: fn() -> (usize, usize),
}

#[macro_export]
macro_rules! solver {
    ($name:expr, $solver:ty, $part_one_test:expr, $part_two_test:expr) => {
        pub const PUZZLE: Puzzle = Puzzle {
            name: $name,
            solve: || {
                let puzzle = include_str!("puzzle.txt");
                (
                    <$solver>::parse(puzzle).part_one(),
                    <$solver>::parse(puzzle).part_two(),
                )
            },
        };

        #[cfg(test)]
        #[test]
        fn test() {
            let test = include_str!("test.txt");
            assert_eq!(<$solver>::parse(test).part_one(), $part_one_test);
            assert_eq!(<$solver>::parse(test).part_two(), $part_two_test);
        }
    };
}
