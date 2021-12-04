pub fn solve() -> (usize, usize) {
    let mut input = load_input(include_str!("input.txt"));
    (
        solve_part1(&input.0, &mut input.1),
        solve_part2(&input.0, &mut input.1),
    )
}

fn load_input(input: &str) -> (Vec<usize>, Vec<Board>) {
    let (draw, cards) = input.split_once("\n").unwrap();
    let draw = draw.split(",").filter_map(|num| num.parse().ok()).collect();
    let lines: Vec<&str> = cards.lines().filter(|l| !l.is_empty()).collect();
    let boards = lines.chunks(5).map(Board::parse).collect();
    (draw, boards)
}

#[derive(Debug)]
pub struct Tile {
    marked: bool,
    num: usize,
}

#[derive(Debug)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
    win_move: usize,
    win_number: Option<usize>,
}

impl Board {
    pub fn parse(board: &[&str]) -> Self {
        let tiles: Vec<Vec<Tile>> = board
            .iter()
            .map(|lile| {
                lile.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .map(|num| Tile { marked: false, num })
                    .collect()
            })
            .collect();
        Self {
            tiles,
            win_move: 0,
            win_number: None,
        }
    }

    pub fn bingo(&self) -> bool {
        self.win_number.is_some()
    }

    pub fn unmarked_sum(&self) -> usize {
        self.tiles
            .iter()
            .map(|l| {
                l.iter()
                    .filter(|tile| !tile.marked)
                    .map(|tile| tile.num)
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn mark(&mut self, move_cnt: usize, num: usize) {
        if self.win_number.is_some() {
            return;
        }

        for line in self.tiles.iter_mut() {
            for tile in line {
                if tile.num == num {
                    tile.marked = true;
                }
            }
        }

        for col in 0..self.tiles[0].len() {
            if self.tiles.iter().all(|l| l[col].marked) {
                self.win_move = move_cnt;
                self.win_number = Some(num);
                break;
            }
        }

        if self.tiles.iter().any(|l| l.iter().all(|tile| tile.marked)) {
            self.win_move = move_cnt;
            self.win_number = Some(num);
        }
    }
}

fn solve_part1(draw: &Vec<usize>, boards: &mut Vec<Board>) -> usize {
    for (i, n) in draw.iter().enumerate() {
        for board in boards.iter_mut() {
            board.mark(i, *n);
            if board.bingo() {
                return n * board.unmarked_sum();
            }
        }
    }
    0
}

fn solve_part2(draw: &Vec<usize>, boards: &mut Vec<Board>) -> usize {
    for (i, n) in draw.iter().enumerate() {
        for board in boards.iter_mut() {
            board.mark(i, *n);
        }
    }

    boards.sort_by(|a, b| a.win_move.partial_cmp(&b.win_move).unwrap());
    let board = boards.iter().last().unwrap();
    board.win_number.unwrap() * board.unmarked_sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_part1() {
        let mut input = super::load_input(include_str!("example.txt"));
        assert_eq!(super::solve_part1(&input.0, &mut input.1), 4512);
    }

    #[test]
    fn solve_part2() {
        let mut input = super::load_input(include_str!("example.txt"));
        assert_eq!(super::solve_part2(&input.0, &mut input.1), 1924);
    }
}
