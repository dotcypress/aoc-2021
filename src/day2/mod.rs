use crate::*;

solver!("Dive!", Submarine, 150, 900);

struct Submarine {
    cmds: Vec<Command>,
}

impl Submarine {
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
        let (cmd, x) = cmd.split_once(" ").unwrap();
        let x = x.parse().unwrap();
        match cmd {
            "forward" => Self::Forward(x),
            "down" => Self::Down(x),
            "up" => Self::Up(x),
            _ => unreachable!(),
        }
    }
}
