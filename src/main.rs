use aoc_2021::*;

fn main() {
    println!("╔══════════════════════════════════╦════════════════╦════════════════╗");
    println!("║ 🦀 Advent of Code 2021           ║       Part One ║       Part Two ║");
    println!("╠══════════════════════════════════╬════════════════╬════════════════╣");
    for puzzle in PUZZLES {
        let name = puzzle.name;
        let (part_one, part_two) = (puzzle.solve)();
        println!("║ {: <32} ║ {: >14} ║ {: >14} ║", name, part_one, part_two);
    }
    println!("╚══════════════════════════════════╩════════════════╩════════════════╝");
}
