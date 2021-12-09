use crate::*;

puzzle!("Day 9: Smoke Basin", SmokeBasin, 15, 0);

struct SmokeBasin {
    heightmap: Vec<Vec<usize>>,
}

impl SmokeBasin {
    fn parse(input: &str) -> Self {
        let heightmap = input
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|ch| ch.to_digit(10))
                    .map(|digit| digit as usize)
                    .collect()
            })
            .collect();
        Self { heightmap }
    }

    fn part_one(&self) -> usize {
        let mut risk_level = 0;
        for row in 0..self.heightmap.len() {
            for col in 0..self.heightmap[row].len() {
                let height = self.heightmap[row][col];
                if Neighbors::at(
                    (col, row),
                    (self.heightmap[row].len(), self.heightmap.len()),
                )
                .all(|(col, row)| self.heightmap[row][col] > height)
                {
                    risk_level += 1 + height;
                }
            }
        }
        risk_level
    }

    fn part_two(&self) -> usize {
        0
    }
}

pub struct Neighbors {
    origin: (usize, usize),
    size: (usize, usize),
    next: usize,
}

impl Neighbors {
    const NEIGHBORHOOD: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    pub fn at(origin: (usize, usize), size: (usize, usize)) -> Self {
        Self {
            origin,
            size,
            next: 0,
        }
    }
}

impl Iterator for Neighbors {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next >= Self::NEIGHBORHOOD.len() {
                return None;
            }
            let addr = Self::NEIGHBORHOOD[self.next];
            self.next += 1;
            let x = self.origin.0 as isize + addr.0;
            let y = self.origin.1 as isize + addr.1;
            if x >= 0 && y >= 0 && x < self.size.0 as isize && y < self.size.1 as isize {
                return Some((x as usize, y as usize));
            }
        }
    }
}
