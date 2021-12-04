use crate::*;

puzzle!(Sonar, 7, 5);

struct Sonar {
    sweep: Vec<usize>,
}

impl Sonar {
    fn scan(&self, window: usize) -> usize {
        self.sweep
            .windows(window + 1)
            .filter(|w| w[0] < w[window])
            .count()
    }
}

impl Puzzle for Sonar {
    fn parse(input: &str) -> Self {
        let sweep = input.lines().filter_map(|line| line.parse().ok()).collect();
        Self { sweep }
    }

    fn part_one(&mut self) -> usize {
        self.scan(1)
    }

    fn part_two(&mut self) -> usize {
        self.scan(3)
    }
}
