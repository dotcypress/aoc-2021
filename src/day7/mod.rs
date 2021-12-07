use crate::*;

puzzle!("Day 7: The Treachery of Whales", CrabSubmarine, 37, 168);

struct CrabSubmarine {
    crabs: Vec<usize>,
}

impl CrabSubmarine {
    fn parse(input: &str) -> Self {
        let crabs = input
            .split(',')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();
        Self { crabs }
    }

    fn part_one(&mut self) -> usize {
        let mut min = usize::MAX;
        let center = self.crabs.iter().sum::<usize>() / self.crabs.len() + 1;
        for crab in 0..=center {
            let cost = self
                .crabs
                .iter()
                .map(|c| ((*c as isize) - crab as isize).abs() as usize)
                .sum();
            if cost < min {
                min = cost;
            }
        }
        min
    }

    fn part_two(&mut self) -> usize {
        let mut min = usize::MAX;
        let center = self.crabs.iter().sum::<usize>() / self.crabs.len() + 1;
        for crab in 0..=center {
            let cost = self
                .crabs
                .iter()
                .map(|c| {
                    let d = ((*c as isize) - crab as isize).abs() as usize;
                    (0..=d).sum::<usize>()
                })
                .sum();
            if cost < min {
                min = cost;
            }
        }
        min
    }
}
