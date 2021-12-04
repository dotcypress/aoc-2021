pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

#[macro_export]
macro_rules! puzzle {
    ($puzzle:ty, $part_one_test:expr, $part_two_test:expr) => {
        pub fn solve() -> (usize, usize) {
            let input = include_str!("puzzle.txt");
            (
                <$puzzle>::parse(input).part_one(),
                <$puzzle>::parse(input).part_two(),
            )
        }

        #[cfg(test)]
        #[test]
        fn test() {
            let input = include_str!("test.txt");
            assert_eq!(<$puzzle>::parse(input).part_one(), $part_one_test);
            assert_eq!(<$puzzle>::parse(input).part_two(), $part_two_test);
        }
    };
}
