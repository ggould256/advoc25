use std::collections::HashMap;

use nalgebra::{Matrix1x2, MatrixXx2};
use regex::Regex;

use crate::parsing::read_regex_records;

type IMatrix = MatrixXx2<i64>;
type IRowVec = Matrix1x2<i64>;
type State = (IMatrix, IMatrix);

fn state_regex() -> Regex {
    Regex::new(concat!(
        r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)",
    )).unwrap()
}

fn parse_states(records: Vec<Vec<String>>) -> State {
    let mut positions: Vec<IRowVec> = Vec::new();
    let mut velocities: Vec<IRowVec> = Vec::new();
    for record in records {
        assert!(record.len() == 5);
        positions.push(IRowVec::new(record[1].parse().unwrap(), record[2].parse().unwrap()));
        velocities.push(IRowVec::new(record[3].parse().unwrap(), record[4].parse().unwrap()));
    }
    (IMatrix::from_rows(&positions), IMatrix::from_rows(&velocities))
}

fn wrap_position(position: IRowVec, w: usize, h: usize) -> IRowVec {
    let w = w as i64;
    let h = h as i64;
    IRowVec::new(((position[0] % w) + w) % w,
                 ((position[1] % h) + h) % h)
}

fn iterate_state(state: &State, w: usize, h:usize, iterations: usize) -> State {
    let (positions, velocities) = state;
    let raw_end_positions: IMatrix = positions + (velocities * (iterations as i64));
    let new_rows: Vec<IRowVec> = 
        raw_end_positions.row_iter()
        .map(|r| wrap_position(r.into(), w, h))
        .collect();
    let end_positions: IMatrix = IMatrix::from_rows(&new_rows);
    (end_positions, velocities.clone())
}

pub fn score_positions(positions: IMatrix, w: usize, h:usize) -> i64 {
    let halfw = (w / 2) as i64;
    let halfh = (h / 2) as i64;
    let mut result = 1;

    result *= positions.row_iter().map(
        |r| (r[0] < halfw && r[1] < halfh) as i64
    ).sum::<i64>();
    result *= positions.row_iter().map(
        |r| (r[0] > halfw && r[1] < halfh) as i64
    ).sum::<i64>();
    result *= positions.row_iter().map(
        |r| (r[0] < halfw && r[1] > halfh) as i64
    ).sum::<i64>();
    result *= positions.row_iter().map(
        |r| (r[0] > halfw && r[1] > halfh) as i64
    ).sum::<i64>();
    
    result    
}

fn print_positions(positions: &IMatrix, w: usize, h:usize) {
    let mut counts: HashMap<(i64, i64), usize> = HashMap::new();
    for row in positions.row_iter() {
        *counts.entry((row[0], row[1])).or_default() += 1;
    }
    for y in 0..h {
        for x in 0..w {
            let count = counts.get(&(x as i64, y as i64)).unwrap_or(&0);
            print!("{}", vec![' ', '.', ':', '-', '+', '*', '#'][*count]);
        }
        println!();
    }
}

fn check_for_tree(positions: &IMatrix) -> bool {
    let xs: Vec<i64> = positions.row_iter().map(|r| r[0]).collect();
    let mut x_counts: HashMap<i64, i64> = HashMap::new();
    for x in xs {
        *x_counts.entry(x).or_default() += 1;
    }
    let max_count = x_counts.values().max().unwrap();
    *max_count > 30
}

pub fn day14_generic(source: Option<String>, w: usize, h:usize, iterations: usize) -> i64 {
    let state_records = read_regex_records(source, state_regex());
    let state = parse_states(state_records);
    let (final_positions, _) = iterate_state(&state, w, h, iterations);
    println!("Positions started as {:?}", state.0);
    println!("Positions ended as {:?}", final_positions);
    score_positions(final_positions, w, h)
}

pub fn day14b_generic(source: Option<String>, w: usize, h:usize) -> i64 {
    let state_records = read_regex_records(source, state_regex());
    let state = parse_states(state_records);
    let mut steps = 0;
    loop {
        let (final_positions, _) = iterate_state(&state, w, h, steps);
        if check_for_tree(&final_positions) {
            print_positions(&final_positions, w, h);
            println!("After {} steps", steps);
        }
        steps += 1;
    }
}

pub fn day14(source: Option<String>) -> i64 {
    day14_generic(source, 101, 103, 100)
}

pub fn day14b(source: Option<String>) -> i64 {
    day14b_generic(source, 101, 103)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day14_generic(Some("data/day14_example.txt".to_string()), 11, 7, 100), 12);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day14(Some("inputs/day14_test.txt".to_string())), 222062148);
    }

    // B cannot be tested.
}
