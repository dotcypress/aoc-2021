fn load_sweep(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
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

pub fn solve() -> (usize, usize) {
    let sweep = load_sweep(include_str!("input.txt"));
    (solve_part1(&sweep), solve_part2(&sweep))
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_part1() {
        let sweep = super::load_sweep(include_str!("example.txt"));
        assert_eq!(super::solve_part1(&sweep), 7);
    }

    #[test]
    fn solve_part2() {
        let sweep = super::load_sweep(include_str!("example.txt"));
        assert_eq!(super::solve_part2(&sweep), 5);
    }
}
