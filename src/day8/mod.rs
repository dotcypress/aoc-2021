use std::collections::HashSet;

use crate::*;

puzzle!(
    "Day 8: Seven Segment Search",
    SevenSegmentDisplay,
    26,
    61229
);

struct SevenSegmentDisplay {
    decoders: Vec<Decoder>,
}

impl SevenSegmentDisplay {
    fn parse(input: &str) -> Self {
        let decoders = input.lines().map(Decoder::parse).collect();
        Self { decoders }
    }

    fn part_one(&self) -> usize {
        self.decoders
            .iter()
            .flat_map(|d| d.decode())
            .filter(|val| *val == 1 || *val == 4 || *val == 7 || *val == 8)
            .count()
    }

    fn part_two(&self) -> usize {
        self.decoders
            .iter()
            .filter_map(|d| d.decode().into_iter().reduce(|acc, x| acc * 10 + x))
            .sum()
    }
}

struct Decoder {
    one: HashSet<char>,
    four: HashSet<char>,
    input: Vec<HashSet<char>>,
}

impl Decoder {
    fn parse(line: &str) -> Self {
        let (patterns, output) = line.split_once('|').unwrap();
        let input = output
            .split_ascii_whitespace()
            .map(|s| s.chars().collect())
            .collect();
        let patterns: Vec<HashSet<char>> = patterns
            .split_ascii_whitespace()
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect();
        let one = patterns.iter().find(|x| x.len() == 2).unwrap().clone();
        let four = patterns.iter().find(|x| x.len() == 4).unwrap().clone();
        Self { input, one, four }
    }

    fn decode(&self) -> Vec<usize> {
        self.input.iter().map(|digit| self.guess(digit)).collect()
    }

    fn guess(&self, digit: &HashSet<char>) -> usize {
        match digit.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            5 if digit.intersection(&self.one).count() == 2 => 3,
            5 if digit.intersection(&self.four).count() == 2 => 2,
            5 if digit.intersection(&self.four).count() == 3 => 5,
            6 if digit.intersection(&self.four).count() == 4 => 9,
            6 if digit.intersection(&self.one).count() == 2 => 0,
            6 => 6,
            _ => 0,
        }
    }
}
