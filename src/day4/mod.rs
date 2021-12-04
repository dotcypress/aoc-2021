use crate::*;

solver!("Day 4: Giant Squid", Bingo, 4512, 1924);
struct Bingo {
    draw: Vec<usize>,
    boards: Vec<Board>,
}

impl Bingo {
    fn parse(input: &str) -> Self {
        let (draw, cards) = input.split_once('\n').unwrap();
        let draw = draw.split(',').filter_map(|num| num.parse().ok()).collect();
        let numbers: Vec<&str> = cards.lines().filter(|l| !l.is_empty()).collect();
        let boards = numbers.chunks(5).map(Board::parse).collect();
        Self { draw, boards }
    }

    fn part_one(self) -> usize {
        let mut boards = self.boards.to_vec();
        for (i, n) in self.draw.iter().enumerate() {
            for board in boards.iter_mut() {
                if board.mark(i, *n) {
                    return board.score();
                }
            }
        }
        0
    }

    fn part_two(self) -> usize {
        let mut boards = self.boards.to_vec();
        for (i, n) in self.draw.iter().enumerate() {
            for board in boards.iter_mut() {
                board.mark(i, *n);
            }
        }
        boards.sort_by(|a, b| a.win_move.partial_cmp(&b.win_move).unwrap());
        boards.iter().last().unwrap().score()
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    marked: bool,
    num: usize,
}

impl Tile {
    pub fn new(num: usize) -> Self {
        Self { num, marked: false }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
    win_number: Option<usize>,
    win_move: usize,
}

impl Board {
    pub fn parse(board: &[&str]) -> Self {
        let tiles = board
            .iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .filter_map(|num| num.parse().ok())
                    .map(Tile::new)
                    .collect()
            })
            .collect();
        Self {
            tiles,
            win_move: 0,
            win_number: None,
        }
    }

    pub fn score(&self) -> usize {
        self.tiles
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|tile| !tile.marked)
                    .map(|tile| tile.num)
                    .sum::<usize>()
            })
            .sum::<usize>()
            * self.win_number.unwrap_or(0)
    }

    pub fn mark(&mut self, move_cnt: usize, num: usize) -> bool {
        if self.win_number.is_some() {
            return true;
        }

        for line in self.tiles.iter_mut() {
            for tile in line {
                if tile.num == num {
                    tile.marked = true;
                    break;
                }
            }
        }

        let bingo = self.tiles.iter().any(|l| l.iter().all(|tile| tile.marked));
        for col in 0..self.tiles[0].len() {
            if bingo || self.tiles.iter().all(|l| l[col].marked) {
                self.win_move = move_cnt;
                self.win_number = Some(num);
                return true;
            }
        }
        false
    }
}
