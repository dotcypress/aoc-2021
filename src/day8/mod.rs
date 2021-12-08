use crate::*;
use std::collections::HashSet;

puzzle!("Day 8: Seven Segment Search", SevenSegment, 26, 61229);

struct SevenSegment {
    decoders: Vec<Decoder>,
}

impl SevenSegment {
    fn parse(input: &str) -> Self {
        let decoders = input.lines().map(Decoder::parse).collect();
        Self { decoders }
    }

    fn part_one(&self) -> usize {
        self.decoders
            .iter()
            .flat_map(|d| d.digits.iter().cloned())
            .filter(|val| *val == 1 || *val == 4 || *val == 7 || *val == 8)
            .count()
    }

    fn part_two(&self) -> usize {
        self.decoders
            .iter()
            .filter_map(|d| d.digits.iter().cloned().reduce(|acc, x| acc * 10 + x))
            .sum()
    }
}

struct Decoder {
    digits: Vec<usize>,
}

impl Decoder {
    fn parse(line: &str) -> Self {
        let (patterns, input) = line.split_once('|').unwrap();
        let input: Vec<HashSet<char>> = input
            .split_ascii_whitespace()
            .map(|s| s.chars().collect())
            .collect();
        let patterns: Vec<HashSet<char>> = patterns
            .split_ascii_whitespace()
            .map(|s| s.chars().collect())
            .collect();
        let one = patterns.iter().find(|x| x.len() == 2).unwrap().clone();
        let four = patterns.iter().find(|x| x.len() == 4).unwrap().clone();
        let digits = input
            .iter()
            .map(|digit| match digit.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                5 if digit.intersection(&one).count() == 2 => 3,
                5 if digit.intersection(&four).count() == 2 => 2,
                5 if digit.intersection(&four).count() == 3 => 5,
                6 if digit.intersection(&four).count() == 4 => 9,
                6 if digit.intersection(&one).count() == 2 => 0,
                6 => 6,
                _ => 0,
            })
            .collect();
        Self { digits }
    }
}
