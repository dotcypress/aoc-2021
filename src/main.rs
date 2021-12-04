use aoc_2021::*;

fn main() {
    for puzzle in PUZZLES {
        let (part_one, part_two) = (puzzle.solve)();
        println!(
            "{}\n├ part one: {}\n└ part two: {}\n",
            puzzle.name, part_one, part_two
        );
    }
}
