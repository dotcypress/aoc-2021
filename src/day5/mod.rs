use crate::*;
use std::collections::HashMap;

puzzle!("Day 5: Hydrothermal Venture", VentsMap, 5, 12);

struct VentsMap {
    vents: Vec<Vent>,
}

impl VentsMap {
    fn parse(input: &str) -> Self {
        Self {
            vents: input.lines().map(Vent::parse).collect(),
        }
    }

    fn part_one(self) -> usize {
        let mut overlaps = HashMap::new();
        for vent in self.vents.iter().filter(|vent| !vent.is_diagonal()) {
            for point in vent.track() {
                *overlaps.entry(point).or_insert(0) += 1;
            }
        }
        overlaps.iter().filter(|x| *x.1 > 1).count()
    }

    fn part_two(self) -> usize {
        let mut overlaps = HashMap::new();
        for vent in self.vents.iter() {
            for point in vent.track() {
                *overlaps.entry(point).or_insert(0) += 1;
            }
        }
        overlaps.iter().filter(|x| *x.1 > 1).count()
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
}

struct Vent {
    from: Point,
    to: Point,
}

impl Vent {
    pub fn parse(input: &str) -> Self {
        let (p1, p2) = input.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(',').unwrap();
        let (x2, y2) = p2.split_once(',').unwrap();
        Self {
            from: Point {
                x: x1.parse().unwrap(),
                y: y1.parse().unwrap(),
            },
            to: Point {
                x: x2.parse().unwrap(),
                y: y2.parse().unwrap(),
            },
        }
    }

    pub fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    pub fn is_diagonal(&self) -> bool {
        (self.from.x - self.to.x).abs() == (self.from.y - self.to.y).abs()
    }

    pub fn track(&self) -> Vec<Point> {
        let mut res = vec![];
        if self.is_vertical() {
            for y in isize::min(self.from.y, self.to.y)..=isize::max(self.from.y, self.to.y) {
                res.push(Point::new(self.from.x, y));
            }
        }
        if self.is_horizontal() {
            for x in isize::min(self.from.x, self.to.x)..=isize::max(self.from.x, self.to.x) {
                res.push(Point::new(x, self.from.y));
            }
        }
        if self.is_diagonal() {
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
