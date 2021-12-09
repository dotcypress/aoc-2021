mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub const PUZZLES: [Puzzle; 9] = [
    day1::PUZZLE,
    day2::PUZZLE,
    day3::PUZZLE,
    day4::PUZZLE,
    day5::PUZZLE,
    day6::PUZZLE,
    day7::PUZZLE,
    day8::PUZZLE,
    day9::PUZZLE,
];

pub struct Puzzle {
    pub name: &'static str,
    pub solve: fn() -> (usize, usize),
}

#[macro_export]
macro_rules! puzzle {
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
