use aoc_2021::*;

fn main() {
    for (day, solver) in SOLVERS.iter().enumerate() {
        let (part_one, part_two) = (solver.solve)();
        println!(
            "Day {}: {}\n├ part one: {}\n└ part two: {}\n",
            day + 1,
            solver.name,
            part_one,
            part_two
        );
    }
}
