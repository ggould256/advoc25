use crate::common::grid_board::{Direction, Xy};
use crate::common::parsing::read_one_string;

type Path = Vec<Direction>;

fn read_input(source: Option<String>) -> Path {
    let input = read_one_string(source);
    let path: Path = input.chars().map(Direction::from_char).collect();
    path
}

pub fn multi_day3(source: Option<String>, n: usize) -> i64 {
    let mut visited: Vec<Xy> = Vec::new();
    let mut positions: Vec<Xy> = vec![];
    for _ in 0..n {
        positions.push(Xy::new(0, 0));
    }
    visited.push(Xy::new(0, 0));
    for (i, direction) in read_input(source).iter().enumerate() {
        positions[i % n] += direction.to_offset();
        visited.push(positions[i % n]);
    }
    let unique_positions: std::collections::HashSet<Xy> = visited.iter().cloned().collect();
    unique_positions.len() as i64
}

pub fn solution_a(source: Option<String>) -> i64 {
    multi_day3(source, 1)
}

pub fn solution_b(source: Option<String>) -> i64 {
    multi_day3(source, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "3";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2015/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2015/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");

    #[test]
    fn test_example_a() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 5);
    }

    #[test]
    fn test_test_a() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 2592);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 13);
    }

    #[test]
    fn test_test_b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 2360);
    }
}
