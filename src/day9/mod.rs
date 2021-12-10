use crate::*;
use std::collections::{HashMap, HashSet};

puzzle!("Day 9: Smoke Basin", SmokeBasin, 15, 1134);

struct SmokeBasin {
    probes: HashMap<Point, usize>,
    width: usize,
    height: usize,
}

impl SmokeBasin {
    fn parse(input: &str) -> Self {
        let mut probes = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let point = Point::new(x as _, y as _);
                probes.insert(point, ch.to_digit(10).unwrap() as _);
                max_x = x;
            }
            max_y = y;
        }
        Self {
            probes,
            width: max_x + 1,
            height: max_y + 1,
        }
    }

    fn part_one(&self) -> usize {
        self.basins()
            .iter()
            .map(|point| self.depth_at(point) + 1)
            .sum()
    }

    fn part_two(&self) -> usize {
        let mut basin_sizes: Vec<usize> = self
            .basins()
            .iter()
            .map(|point| {
                let mut flood = HashSet::new();
                self.fill_basin(point, &mut flood);
                flood.len()
            })
            .collect();
        basin_sizes.sort_unstable();
        basin_sizes.iter().rev().take(3).product()
    }

    fn basins(&self) -> Vec<Point> {
        let mut res = vec![];
        for row in 0..self.height {
            for col in 0..self.width {
                let point = Point::new(col as _, row as _);
                let min = self.depth_at(&point);
                let is_basin = self
                    .neighbors_at(&point)
                    .iter()
                    .all(|neighbor| self.depth_at(neighbor) > min);
                if is_basin {
                    res.push(point);
                }
            }
        }
        res
    }

    fn fill_basin(&self, point: &Point, flood: &mut HashSet<Point>) {
        if flood.contains(point) || self.depth_at(point) == 9 {
            return;
        }
        flood.insert(*point);
        for neighbor in self.neighbors_at(point) {
            self.fill_basin(&neighbor, flood)
        }
    }

    fn depth_at(&self, point: &Point) -> usize {
        *self.probes.get(point).unwrap()
    }

    fn neighbors_at(&self, point: &Point) -> Vec<Point> {
        [
            Point::new(-1, 0),
            Point::new(0, 1),
            Point::new(1, 0),
            Point::new(0, -1),
        ]
        .iter()
        .filter_map(|neighbor| {
            let heighbor = Point::new(point.x + neighbor.x, point.y + neighbor.y);
            self.probes.get(&heighbor).map(|_| heighbor)
        })
        .collect()
    }
}
