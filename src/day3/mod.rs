use crate::*;
use std::collections::*;

solver!("Binary Diagnostic", NoiseAnalyzer, 198, 230);

struct NoiseAnalyzer {
    bits: usize,
    report: Vec<usize>,
}

impl NoiseAnalyzer {
    fn parse(input: &str) -> Self {
        let bits = input.lines().take(1).last().unwrap().len();
        let report = input
            .lines()
            .filter_map(|line| usize::from_str_radix(line, 2).ok())
            .collect();
        Self { bits, report }
    }

    fn part_one(self) -> usize {
        let mut gamma = 0;
        let mut epsilon = 0;
        for i in 0..self.bits {
            gamma <<= 1;
            epsilon <<= 1;
            match Self::bit_stats(self.bits - i, self.report.iter().copied()) {
                (zeros, ones) if zeros > ones => epsilon += 1,
                _ => gamma += 1,
            }
        }
        gamma * epsilon
    }

    fn part_two(self) -> usize {
        self.rating(false) * self.rating(true)
    }

    fn bit_stats<I: ExactSizeIterator<Item = usize>>(bit: usize, report: I) -> (usize, usize) {
        let total = report.len();
        let mask = 1 << (bit - 1);
        let ones = report.filter(|line| (*line & mask) == mask).count();
        (total - ones, ones)
    }

    fn rating(&self, inv: bool) -> usize {
        let mut set = HashSet::<usize>::from_iter(self.report.iter().copied());
        for i in 0..self.bits {
            let (zeros, ones) = Self::bit_stats(self.bits - i, set.iter().copied());
            let mask = 1 << (self.bits - i - 1);
            let criteria = (ones >= zeros) ^ inv;
            set.retain(|report| criteria == ((report & mask) == mask));
            if set.len() == 1 {
                break;
            }
        }
        *set.iter().last().unwrap()
    }
}
