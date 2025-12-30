// NOTE this module is incomplete and does not pass.

use log::debug;

use crate::common::parsing::read_all_records;
use crate::common::arraylike::{transpose, transpose_strings};

#[derive(Debug, Clone)]
enum Op { Add, Mul }
impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("Unknown operation {}", s),
        }
    }
}

#[derive(Debug, Clone)]
struct PuzzleInput {
    numbers: Vec<Vec<i64>>,
    ops: Vec<Op>,
}

fn read_input(source: Option<String>) -> PuzzleInput {
    let lines = read_all_records(source);
    let mut result = PuzzleInput {
        numbers: Vec::new(),
        ops: Vec::new(),
    };
    for line in lines {
        if line[0].parse::<i64>().is_ok() {
            let num_row = line.iter().map(|s| s.parse::<i64>().unwrap()).collect();
            result.numbers.push(num_row);
        } else {
            result.ops = line.iter().map(|s| Op::from(s.as_str())).collect();
        }
    }
    result
}

fn read_input_transposed(source: Option<String>) -> PuzzleInput {
    let lines: Vec<Vec<String>> = read_all_records(source);
    let mut numbers_strings = Vec::<Vec<String>>::new();
    let mut result = PuzzleInput {
        numbers: Vec::new(),
        ops: Vec::new(),
    };
    for line in lines {
        if line[0].parse::<i64>().is_ok() {
            numbers_strings.push(line);
        } else {
            result.ops = line.iter().map(|s| Op::from(s.as_str())).collect();
        }
    }
    let numbers_strings_rows = transpose(&numbers_strings[..]);
    for row in numbers_strings_rows {
        let transposed_numbers_strings = transpose_strings(
            row.iter().map(|s| s.as_str()).collect::<Vec<&str>>().as_slice());
        result.numbers.push(
            transposed_numbers_strings
                .iter()
                .map(|s| s.parse::<i64>().unwrap())
                .collect(),
        );
    }
    result
}

fn col_totals(numbers: &[Vec<i64>], ops: &[Op]) -> Vec<i64> {
    let mut totals = Vec::new();
    for (col_idx, op) in ops.iter().enumerate() {
        debug!("Processing column {:?} with operation {:?}", numbers[col_idx], op);
        let col = &numbers[col_idx];
        match op {
            Op::Add => {
                totals.push(col.iter().sum());
            }
            Op::Mul => {
                totals.push(col.iter().product());
            }
        }
    }
    totals
}


pub fn solution_a(source: Option<String>) -> i64 {
    let puzzle = read_input(source);
    debug!("Parsed puzzle input: {:?}", puzzle);
    let mut pivoted_numbers: Vec<Vec<i64>> = vec![vec![]; puzzle.numbers[0].len()];
    for row in puzzle.numbers {
        for (i, &num) in row.iter().enumerate() {
            pivoted_numbers[i].push(num);
        }
    }
    let col_totals = col_totals(&pivoted_numbers, &puzzle.ops);
    col_totals.iter().sum()
}

pub fn solution_b(source: Option<String>) -> i64 {
    let puzzle = read_input_transposed(source);
    debug!("Parsed puzzle input: {:?}", puzzle);
    let col_totals = col_totals(&puzzle.numbers, &puzzle.ops);
    col_totals.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "6";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");

    #[test]
    #[ignore = "This module is incomplete and does not pass."]
    fn test_example() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 4277556);
    }

    #[test]
    #[ignore = "This module is incomplete and does not pass."]
    fn test_test_a() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 733);
    }

    #[test]
    #[ignore = "This module is incomplete and does not pass."]
    fn test_example_b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 14);
    }

    #[test]
    #[ignore = "This module is incomplete and does not pass."]
    fn test_test_b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 345821388687084);
    }
}
