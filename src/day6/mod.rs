use crate::*;

puzzle!("Day 6: Lanternfish", Lanternfish, 5934, 26984457539);

struct Lanternfish {
    gens: [usize; 9],
}

impl Lanternfish {
    fn parse(input: &str) -> Self {
        let gens =
            input
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .fold([0; 9], |mut gens, fish| {
                    gens[fish] += 1;
                    gens
                });
        Self { gens }
    }

    fn population(&mut self, days: usize) -> usize {
        for _ in 0..days {
            let respawn = self.gens[0];
            for gen in 1..=8 {
                self.gens[gen - 1] = self.gens[gen];
            }
            self.gens[6] += respawn;
            self.gens[8] = respawn;
        }
        self.gens.iter().sum()
    }

    fn part_one(&mut self) -> usize {
        self.population(80)
    }

    fn part_two(&mut self) -> usize {
        self.population(256)
    }
}
