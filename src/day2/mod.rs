pub enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Command {
    pub fn parse(cmd: &str) -> Self {
        match cmd.split_once(" ") {
            Some(("forward", n)) => Self::Forward(n.parse().unwrap()),
            Some(("down", n)) => Self::Down(n.parse().unwrap()),
            Some(("up", n)) => Self::Up(n.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

fn load_commands(input: &str) -> Vec<Command> {
    input.lines().map(Command::parse).collect()
}

fn solve_part1(cmds: &Vec<Command>) -> usize {
    let mut pos = 0;
    let mut depth = 0;
    for cmd in cmds {
        match cmd {
            Command::Down(x) => depth += x,
            Command::Up(x) => depth -= x,
            Command::Forward(x) => pos += x,
        }
    }
    pos * depth
}

fn solve_part2(cmds: &Vec<Command>) -> usize {
    let mut aim = 0;
    let mut pos = 0;
    let mut depth = 0;
    for cmd in cmds {
        match cmd {
            Command::Down(x) => aim += x,
            Command::Up(x) => aim -= x,
            Command::Forward(x) => {
                pos += x;
                depth += aim * x;
            }
        }
    }
    pos * depth
}

pub fn solve() -> (usize, usize) {
    let cmds = load_commands(include_str!("input.txt"));
    (solve_part1(&cmds), solve_part2(&cmds))
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_part1() {
        let cmds = super::load_commands(include_str!("example.txt"));
        assert_eq!(super::solve_part1(&cmds), 150);
    }

    #[test]
    fn solve_part2() {
        let cmds = super::load_commands(include_str!("example.txt"));
        assert_eq!(super::solve_part2(&cmds), 900);
    }
}
