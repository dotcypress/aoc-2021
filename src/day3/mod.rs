use std::collections::*;

pub fn solve() -> (usize, usize) {
    let (bits, reports) = load_input(include_str!("input.txt"));
    (solve_part1(bits, &reports), solve_part2(bits, &reports))
}

fn load_input(input: &str) -> (usize, Vec<usize>) {
    (
        input.lines().take(1).last().unwrap().len(),
        input
            .lines()
            .filter_map(|line| usize::from_str_radix(line, 2).ok())
            .collect(),
    )
}

fn calc_bit_stats<I: ExactSizeIterator<Item = usize>>(bit: usize, reports: I) -> (usize, usize) {
    let total = reports.len();
    let mask = 1 << (bit - 1);
    let ones = reports.filter(|line| (*line & mask) == mask).count();
    (total - ones, ones)
}

fn calc_rating(bits: usize, inv: bool, reports: &Vec<usize>) -> usize {
    let mut set = HashSet::<usize>::from_iter(reports.iter().copied());
    for i in 0..bits {
        let (zeros, ones) = calc_bit_stats(bits - i, set.iter().copied());
        let mask = 1 << (bits - i - 1);
        let criteria = (ones >= zeros) ^ inv;
        set.retain(|report| criteria == ((report & mask) == mask));
        if set.len() == 1 {
            break;
        }
    }
    *set.iter().last().unwrap()
}

fn solve_part1(bits: usize, reports: &Vec<usize>) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..bits {
        gamma <<= 1;
        epsilon <<= 1;
        match calc_bit_stats(bits - i, reports.iter().copied()) {
            (zeros, ones) if zeros > ones => epsilon += 1,
            _ => gamma += 1,
        }
    }
    gamma * epsilon
}

fn solve_part2(bits: usize, reports: &Vec<usize>) -> usize {
    calc_rating(bits, false, reports) * calc_rating(bits, true, reports)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_part1() {
        let (bits, reports) = super::load_input(include_str!("example.txt"));
        assert_eq!(super::solve_part1(bits, &reports), 198);
    }

    #[test]
    fn solve_part2() {
        let (bits, reports) = super::load_input(include_str!("example.txt"));
        assert_eq!(super::solve_part2(bits, &reports), 230);
    }
}
