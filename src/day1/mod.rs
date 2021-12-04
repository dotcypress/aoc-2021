use crate::*;

solver!("Day 1: Sonar Sweep", Sonar, 7, 5);

struct Sonar {
    sweep: Vec<usize>,
}

impl Sonar {
    fn parse(input: &str) -> Self {
        let sweep = input.lines().filter_map(|line| line.parse().ok()).collect();
        Self { sweep }
    }

    fn part_one(self) -> usize {
        self.scan(1)
    }

    fn part_two(self) -> usize {
        self.scan(3)
    }

    fn scan(&self, window: usize) -> usize {
        self.sweep
            .windows(window + 1)
            .filter(|w| w[0] < w[window])
            .count()
    }
}
