
use log::debug;

use crate::common::grid_board::{self, Xy};
use crate::common::parsing::read_grid_board;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Start,
    Splitter,
    Beam,
}

impl TryInto<char> for Cell {
    fn try_into(self) -> Result<char, Self::Error> {
        match self {
            Cell::Empty => Ok('.'),
            Cell::Start => Ok('S'),
            Cell::Splitter => Ok('^'),
            Cell::Beam => Ok('|'),
        }
    }
    
    type Error = (); // No error possible in this conversion
}

impl TryFrom<char> for Cell {
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Empty),
            'S' => Ok(Cell::Start),
            '^' => Ok(Cell::Splitter),
            '|' => Ok(Cell::Beam),
            _ => Err(()),
        }
    }

    type Error = ();
}

type Board = grid_board::Board<Cell>;

fn read_input(source: Option<String>) -> Board {
    let board: Board = read_grid_board(source);
    debug!("Read board: \n{:?}", board);
    board
}

fn iterate(board: &Board) -> Board {
    let start_kernel = Board::opt_from_strs(&vec![
        "S",
        ".",
    ]);
    let start_result = Board::opt_from_strs(&vec![
        "S",
        "|",
    ]);
    let split_kernel = Board::opt_from_strs(&vec![
        "_|_",
        "_^_",
    ]);
    let split_result = Board::opt_from_strs(&vec![
        "_|_",
        "|^|",
    ]);
    let extend_kernel = Board::opt_from_strs(&vec![
        "|",
        ".",
    ]);
    let extend_result = Board::opt_from_strs(&vec![
        "|",
        "|",
    ]);
    board
        .transform(&start_kernel, &start_result)
        .transform(&split_kernel, &split_result)
        .transform(&extend_kernel, &extend_result)
}

fn iterate_until_stable(board: &Board) -> Board {
    let mut board = board.clone();
    loop {
        let new_board = iterate(&board);
        if new_board == board {
            debug!("DONE");
            return new_board;
        }
        board = new_board;
    }
}

fn solutions(source: Option<String>) -> (i64, i64) {
    let board = read_input(source);
    let board = iterate_until_stable(&board);
    let split_result = Board::opt_from_strs(&vec![
        "|",
        "^",
    ]);

    let mut timeline_counts: grid_board::Board<usize> = grid_board::Board::new(
        board.height(),
        board.width(),
        0,
    );
    for y in 0..board.height() {
        for (x, content) in board.row(y).iter().enumerate() {
            let xy = Xy::new(x as i64, y as i64);
            let count = timeline_counts.at(xy);
            debug!("At position {:?} with content {:?} and count {}", xy, content, count);
            match content {
                Cell::Start => {
                    timeline_counts.set_at(xy, 1);
                }
                Cell::Splitter => {
                    *timeline_counts.mut_at(xy + Xy::new(-1, 0)) += count;
                    *timeline_counts.mut_at(xy + Xy::new(1, 0)) += count;
                }
                _ => {}
            }
        }
        for (x, content) in board.row(y).iter().enumerate() {
            let xy = Xy::new(x as i64, y as i64);
            let count = timeline_counts.at(xy);
            match content {
                Cell::Beam  | Cell::Start => {
                    if timeline_counts.maybe_at(xy + Xy::new(0, 1)).is_some() {
                        *timeline_counts.mut_at(xy + Xy::new(0, 1)) += count;
                    }
                }
                _ => {}
            }
        }
    }
    (board.scan(&split_result).count(true) as i64,
     timeline_counts.row(board.height() - 1).iter().sum::<usize>() as i64)
}

pub fn solution_a(source: Option<String>) -> i64 {
    solutions(source).0 as i64
}

pub fn solution_b(source: Option<String>) -> i64 {
    solutions(source).1 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "7";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");

    #[test]
    fn test_example() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 21);
    }

    #[test]
    fn test_test_a() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 1541);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 40);
    }

    #[test]
    fn test_test_b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 80158285728929);
    }
}
