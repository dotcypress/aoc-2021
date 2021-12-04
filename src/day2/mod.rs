use crate::*;

puzzle!(Dive, 150, 900);

struct Dive {
    cmds: Vec<Command>,
}

impl Dive {
    fn parse(input: &str) -> Self {
        let cmds = input.lines().map(Command::parse).collect();
        Self { cmds }
    }

    fn part_one(self) -> usize {
        let mut pos = 0;
        let mut depth = 0;
        for cmd in &self.cmds {
            match cmd {
                Command::Down(x) => depth += x,
                Command::Up(x) => depth -= x,
                Command::Forward(x) => pos += x,
            }
        }
        pos * depth
    }

    fn part_two(self) -> usize {
        let mut aim = 0;
        let mut pos = 0;
        let mut depth = 0;
        for cmd in &self.cmds {
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
}

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
