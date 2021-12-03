use std::collections::*;

pub fn solve() -> (usize, usize) {
    let input = load_input(include_str!("input.txt"));
    (solve_part1(12, &input), solve_part2(12, &input))
}

fn load_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter_map(|line| usize::from_str_radix(line, 2).ok())
        .collect()
}

fn calc_stats(len: usize, input: &Vec<usize>) -> HashMap<usize, (usize, usize)> {
    let mut stats = HashMap::new();
    for i in 0..len {
        let mask = 1 << (len - i - 1);
        let mut ones = 0;
        for line in input {
            if (line & mask) == mask {
                ones += 1;
            }
        }
        stats.insert(i, (input.len() - ones, ones));
    }
    stats
}

fn calc_rating(len: usize, inverse: bool, input: &Vec<usize>) -> usize {
    let mut set = HashSet::<usize>::from_iter(input.clone());
    for i in 0..len {
        let stats = calc_stats(len, &set.iter().cloned().collect());
        let (zeros, ones) = stats.get(&i).unwrap();
        let mask = 1 << (len - i - 1);
        for line in input {
            let bit_set = ((line & mask) == mask) ^ inverse;
            if (ones >= zeros && !bit_set) || (ones < zeros && bit_set) {
                set.remove(line);
            }
        }
        if set.len() == 1 {
            break;
        }
    }
    *set.iter().last().unwrap()
}

fn solve_part1(len: usize, input: &Vec<usize>) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;
    let stats = calc_stats(len, input);
    for i in 0..len {
        gamma <<= 1;
        epsilon <<= 1;
        let (zeros, ones) = stats.get(&i).unwrap();
        if zeros > ones {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    }
    gamma * epsilon
}

fn solve_part2(len: usize, input: &Vec<usize>) -> usize {
    calc_rating(len, false, input) * calc_rating(len, true, input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_part1() {
        let input = super::load_input(include_str!("example.txt"));
        assert_eq!(super::solve_part1(5, &input), 198);
    }

    #[test]
    fn solve_part2() {
        let input = super::load_input(include_str!("example.txt"));
        assert_eq!(super::solve_part2(5, &input), 230);
    }
}
