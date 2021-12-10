use crate::*;

puzzle!("Day 10: Syntax Scoring", Parser, 26397, 288957);

struct Parser {
    lines: Vec<String>,
}

impl Parser {
    fn parse(input: &str) -> Self {
        let lines = input.lines().map(|line| line.to_string()).collect();
        Self { lines }
    }

    fn part_one(&self) -> usize {
        self.lines
            .iter()
            .map(|line| match complete(line) {
                Ok(_) => 0,
                Err(token) => match token {
                    '(' => 3,
                    '[' => 57,
                    '{' => 1197,
                    '<' => 25137,
                    _ => 0,
                },
            })
            .sum()
    }

    fn part_two(&self) -> usize {
        let mut scores: Vec<usize> = self
            .lines
            .iter()
            .filter_map(|line| complete(line).ok())
            .map(|completion| {
                completion
                    .iter()
                    .map(|token| match token {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    })
                    .fold(0, |acc, score| score + acc * 5)
            })
            .collect();
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

fn complete(line: &str) -> Result<Vec<char>, char> {
    let mut completion = vec![];
    for token in line.chars() {
        match token {
            '(' | '[' | '{' | '<' => {
                completion.push(token);
            }
            ')' => {
                if completion.pop() != Some('(') {
                    return Err('(');
                }
            }
            ']' => {
                if completion.pop() != Some('[') {
                    return Err('[');
                }
            }
            '}' => {
                if completion.pop() != Some('{') {
                    return Err('{');
                }
            }
            '>' => {
                if completion.pop() != Some('<') {
                    return Err('<');
                }
            }
            _ => {}
        }
    }
    completion.reverse();
    Ok(completion)
}
