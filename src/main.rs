use aoc_2021::*;

fn main() {
    println!("╔══════════════════════════════════╦════════════════╦════════════════╗\n║ 🎄 Advent of Code \x1b[35m2021\x1b[0m           ║       Part One ║       Part Two ║\n╠══════════════════════════════════╬════════════════╬════════════════╣");
    for puzzle in PUZZLES {
        let (part_one, part_two) = (puzzle.solve)();
        println!(
            "║ {: <32} ║ {: >14} ║ {: >14} ║",
            puzzle.name, part_one, part_two
        );
    }
    println!("╚══════════════════════════════════╩════════════════╩════════════════╝");
}
