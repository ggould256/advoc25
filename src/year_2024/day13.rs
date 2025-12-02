
use nalgebra::{SMatrix, Vector2};
use regex::Regex;

use crate::parsing::read_regex_records;

type Scalar = i64;
type Coords = (Scalar, Scalar);

#[derive(Debug, Clone)]
struct Puzzle {
    buttons: (Coords, Coords),
    prize: Coords
}

fn puzzle_regex() -> Regex {
    Regex::new(concat!(
        r"Button A: X\+(\d+), Y\+(\d+)\n",
        r"Button B: X\+(\d+), Y\+(\d+)\n",
        r"Prize: X=(\d+), Y=(\d+)\n",
    )).unwrap()
}

fn parse_puzzles(records: Vec<Vec<String>>) -> Vec<Puzzle> {
    let mut result = Vec::new();
    for puzzle_record in records {
        assert!(puzzle_record.len() == 7);
        result.push(
            Puzzle{buttons:((puzzle_record[1].parse().unwrap(), puzzle_record[2].parse().unwrap()),
                            (puzzle_record[3].parse().unwrap(), puzzle_record[4].parse().unwrap())),
                   prize:(puzzle_record[5].parse().unwrap(), puzzle_record[6].parse().unwrap())});
    }
    result
}

type Mat2r = SMatrix<f64, 2, 2>;


fn puzzle_cost(puzzle: Puzzle) -> Option<i64> {
    let button_matrix =  // Call this "B"
        Mat2r::new(
            puzzle.buttons.0.0 as f64, puzzle.buttons.1.0 as f64,
            puzzle.buttons.0.1 as f64, puzzle.buttons.1.1 as f64);
    let prize_vector =  // Call this "p"
        Vector2::<f64>::new(puzzle.prize.0 as f64, puzzle.prize.1 as f64);
    let inverse_button_matrix =  // Call this Bi
        button_matrix.try_inverse()?;
    // Press buttons b to get prize p:  B * b = p
    // to get prize p, press buttons b:  b = Bi * p
    // to price buttons b:  $$$ = (Bi * p)' . [3 1]
    let required_buttons = inverse_button_matrix * prize_vector;

    let button_matrix = button_matrix.map(|f| f.round() as i64);
    let required_buttons = required_buttons.map(|f| f.round() as i64);
    let prize_vector = prize_vector.map(|f| f.round() as i64);
    let endpoint = button_matrix * required_buttons;
    println!("To achieve prize {:?} with buttons {:?}, press {:?}",
             prize_vector, button_matrix, required_buttons);
    if endpoint == prize_vector {
        let cost = Vector2::<i64>::new(3, 1);
        let score = required_buttons.transpose() * cost;
        Some(score[(0, 0)])    
    } else {
        println!("Which misses the prize.");
        None
    }
}

pub fn day13(source: Option<String>) -> i64 {
    let mut result: i64 = 0;
    let puzzle_records = read_regex_records(source, puzzle_regex());
    let puzzles = parse_puzzles(puzzle_records);
    for puzzle in puzzles {
        println!("{:?}", puzzle);
        result += puzzle_cost(puzzle).unwrap_or_default();
    }
    result
}

pub fn day13b(source: Option<String>) -> i64 {
    let mut result: i64 = 0;
    let puzzle_records = read_regex_records(source, puzzle_regex());
    let puzzles = parse_puzzles(puzzle_records);
    for puzzle in puzzles {
        println!("{:?}", puzzle);
        let mut puzzle = puzzle.clone();
        puzzle.prize.0 += 10000000000000;
        puzzle.prize.1 += 10000000000000;
        result += puzzle_cost(puzzle).unwrap_or_default();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example2() {
        assert_eq!(day13(Some("data/day13_example.txt".to_string())), 480);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day13(Some("inputs/day13_test.txt".to_string())), 31589);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day13b(Some("data/day13_example.txt".to_string())), 875318608908);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(day13b(Some("inputs/day13_test.txt".to_string())), 98080815200063);
    }
}
