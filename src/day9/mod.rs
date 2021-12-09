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
        self.basins()
            .iter()
            .map(|point| self.heightmap[point.y as usize][point.x as usize] + 1)
            .sum()
    }

    fn part_two(&self) -> usize {
        0
    }

    fn basins(&self) -> Vec<Point> {
        let mut res = vec![];
        let map_size = (self.heightmap[0].len(), self.heightmap.len());
        for row in 0..self.heightmap.len() {
            for col in 0..self.heightmap[row].len() {
                let at = Point::new(col as isize, row as isize);
                let height = self.height_at(at);
                if Neighbors::at(at, map_size).all(|point| self.height_at(point) > height) {
                    res.push(at);
                }
            }
        }
        res
    }

    pub fn height_at(&self, origin: Point) -> usize {
        self.heightmap[origin.y as usize][origin.x as usize]
    }
}

pub struct Neighbors {
    origin: Point,
    size: (usize, usize),
    next: usize,
}

impl Neighbors {
    const NEIGHBORHOOD: [Point; 4] = [
        Point::new(-1, 0),
        Point::new(0, 1),
        Point::new(1, 0),
        Point::new(0, -1),
    ];

    pub fn at(origin: Point, size: (usize, usize)) -> Self {
        Self {
            origin,
            size,
            next: 0,
        }
    }
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next >= Self::NEIGHBORHOOD.len() {
                return None;
            }
            let addr = &Self::NEIGHBORHOOD[self.next];
            self.next += 1;
            let x = self.origin.x + addr.x;
            let y = self.origin.y + addr.y;
            if x >= 0 && y >= 0 && x < self.size.0 as isize && y < self.size.1 as isize {
                return Some(Point::new(x, y));
            }
        }
    }
}
