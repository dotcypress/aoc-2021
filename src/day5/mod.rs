use crate::*;
use std::collections::HashMap;

puzzle!("Day 5: Hydrothermal Venture", VentsMap, 5, 12);

struct VentsMap {
    vents: Vec<Vent>,
}

impl VentsMap {
    fn parse(input: &str) -> Self {
        let vents = input.lines().map(Vent::parse).collect();
        Self { vents }
    }

    fn overlaps(&self, include_diagonals: bool) -> usize {
        self.vents
            .iter()
            .filter(|vent| include_diagonals || !vent.is_diagonal())
            .flat_map(|v| v.track())
            .fold(HashMap::new(), |mut acc, point| {
                *acc.entry(point).or_insert(0) += 1;
                acc
            })
            .iter()
            .filter(|x| *x.1 > 1)
            .count()
    }

    fn part_one(self) -> usize {
        self.overlaps(false)
    }

    fn part_two(self) -> usize {
        self.overlaps(true)
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn parse(input: &str) -> Self {
        let (x, y) = input.split_once(',').unwrap();
        Self::new(x.parse().unwrap(), y.parse().unwrap())
    }
}

struct Vent {
    from: Point,
    to: Point,
}

impl Vent {
    pub fn parse(input: &str) -> Self {
        let (p1, p2) = input.split_once(" -> ").unwrap();
        Self {
            from: Point::parse(p1),
            to: Point::parse(p2),
        }
    }

    pub fn is_diagonal(&self) -> bool {
        (self.from.x - self.to.x).abs() == (self.from.y - self.to.y).abs()
    }

    pub fn track(&self) -> Vec<Point> {
        let mut res = vec![];
        if self.from.x == self.to.x {
            for y in isize::min(self.from.y, self.to.y)..=isize::max(self.from.y, self.to.y) {
                res.push(Point::new(self.from.x, y));
            }
        } else if self.from.y == self.to.y {
            for x in isize::min(self.from.x, self.to.x)..=isize::max(self.from.x, self.to.x) {
                res.push(Point::new(x, self.from.y));
            }
        } else {
            let len = (self.to.x - self.from.x).abs();
            let step_x = (self.to.x - self.from.x).signum();
            let step_y = (self.to.y - self.from.y).signum();
            let mut x = self.from.x;
            let mut y = self.from.y;
            for _ in 0..=len {
                res.push(Point::new(x, y));
                x += step_x;
                y += step_y;
            }
        }
        res
    }
}
