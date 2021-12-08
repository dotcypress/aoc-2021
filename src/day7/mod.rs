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

    fn min_fuel_cost(&self, engine: fn(usize) -> usize) -> usize {
        let mut min = usize::MAX;
        let avg = self.crabs.iter().sum::<usize>() / self.crabs.len() + 1;
        for pos in 0..=avg {
            let cost = self
                .crabs
                .iter()
                .map(|crab| ((*crab as isize) - pos as isize).abs() as usize)
                .map(engine)
                .sum();
            if cost < min {
                min = cost;
            }
        }
        min
    }

    fn part_one(&self) -> usize {
        self.min_fuel_cost(|s| s)
    }

    fn part_two(&self) -> usize {
        self.min_fuel_cost(|s| (0..=s).sum())
    }
}
