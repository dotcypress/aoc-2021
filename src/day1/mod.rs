pub fn solve() -> (usize, usize) {
    let input = load_input(include_str!("input.txt"));
    (solve_part1(&input), solve_part2(&input))
}

fn load_input(input: &str) -> Vec<usize> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn scan(sweep: &Vec<usize>, window: usize) -> usize {
    sweep
        .windows(window + 1)
        .filter(|w| w[0] < w[window])
        .count()
}

fn solve_part1(sweep: &Vec<usize>) -> usize {
    scan(sweep, 1)
}

fn solve_part2(sweep: &Vec<usize>) -> usize {
    scan(sweep, 3)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_part1() {
        let input = super::load_input(include_str!("example.txt"));
        assert_eq!(super::solve_part1(&input), 7);
    }

    #[test]
    fn solve_part2() {
        let input = super::load_input(include_str!("example.txt"));
        assert_eq!(super::solve_part2(&input), 5);
    }
}
