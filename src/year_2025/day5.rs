use log::debug;
use regex::Regex;

use crate::common::parsing::read_regex_records;

type Id = usize;
struct IdRange {
    start: Id,
    end: Id,
}

fn read_input(source: Option<String>) -> (Vec<IdRange>, Vec<Id>) {
    let inventory_re = Regex::new(r"(\d+)(?:-(\d+))?").unwrap();
    let lines = read_regex_records(source, inventory_re);
    let mut ranges: Vec<IdRange> = Vec::new();
    let mut ids: Vec<Id> = Vec::new();
    for record in &lines {
        if record.len() == 2 {
            let id: Id = record[0].parse().unwrap();
            ids.push(id);
        } else if record.len() == 3 {
            let start: Id = record[1].parse().unwrap();
            let end: Id = record[2].parse().unwrap();
            ranges.push(IdRange { start, end });
        } else {
            panic!("Unexpected record length {}", record.len());
        }
    }
    (ranges, ids)
}

fn solutions(source: Option<String>) -> (i64, i64) {
    let (ranges, ids) = read_input(source);
    let mut in_ranges = 0;
    for id in ids {
        for range in &ranges {
            if range.start <= id && id <= range.end {
                debug!("Id {} is in range {}-{}", id, range.start, range.end);
                in_ranges += 1;
                break;
            }
        }
    }
    (in_ranges, 0)
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

    const DAY: &str = "5";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");

    #[test]
    fn test_example() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 3);
    }

    #[test]
    fn test_test_a() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 733);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 0);
    }

    #[test]
    fn test_test_b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 0);
    }
}
