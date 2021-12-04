use crate::*;
use std::collections::*;

puzzle!(BinaryDiagnostic, 198, 230);

struct BinaryDiagnostic {
    bits: usize,
    reports: Vec<usize>,
}

impl BinaryDiagnostic {
    fn parse(input: &str) -> Self {
        let bits = input.lines().take(1).last().unwrap().len();
        let reports = input
            .lines()
            .filter_map(|line| usize::from_str_radix(line, 2).ok())
            .collect();
        Self { bits, reports }
    }

    fn part_one(self) -> usize {
        let mut gamma = 0;
        let mut epsilon = 0;
        for i in 0..self.bits {
            gamma <<= 1;
            epsilon <<= 1;
            match Self::bit_stats(self.bits - i, self.reports.iter().copied()) {
                (zeros, ones) if zeros > ones => epsilon += 1,
                _ => gamma += 1,
            }
        }
        gamma * epsilon
    }

    fn part_two(self) -> usize {
        self.calc_rating(false) * self.calc_rating(true)
    }

    fn bit_stats<I: ExactSizeIterator<Item = usize>>(bit: usize, reports: I) -> (usize, usize) {
        let total = reports.len();
        let mask = 1 << (bit - 1);
        let ones = reports.filter(|line| (*line & mask) == mask).count();
        (total - ones, ones)
    }

    fn calc_rating(&self, inv: bool) -> usize {
        let mut set = HashSet::<usize>::from_iter(self.reports.iter().copied());
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
