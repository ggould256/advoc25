use log::debug;
use regex::Regex;

use crate::common::parsing::read_regex_records;

type Id = usize;
struct IdRange {
    start: Id,
    end: Id,
}

enum EndpointKind { Start, End }

struct RangePoint {
    id: Id,
    kind: EndpointKind,
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
                in_ranges += 1;
                break;
            }
        }
    }

    let mut range_points: Vec<RangePoint> = vec![];
    for range in ranges {
        range_points.push(RangePoint {
            id: range.start,
            kind: EndpointKind::Start,
        });
        range_points.push(RangePoint {
            id: range.end,
            kind: EndpointKind::End,
        });
    }
    range_points.sort_by(|a, b| {
        if a.id != b.id {
            a.id.cmp(&b.id)
        } else {
            match (&a.kind, &b.kind) {
                (EndpointKind::Start, EndpointKind::End) => std::cmp::Ordering::Less,
                (EndpointKind::End, EndpointKind::Start) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        }
    });
    let mut num_ranges_open = 0;
    let mut range_start: Option<Id> = None;
    let mut total_covered = 0;
    for RangePoint { id, kind } in range_points {
        match kind {
            EndpointKind::Start => {
                num_ranges_open += 1;
                debug!("At id {}: opened range, now {} open", id, num_ranges_open);
            }
            EndpointKind::End => { 
                num_ranges_open -= 1;
                debug!("At id {}: closed range, now {} open", id, num_ranges_open);
            }
        }
        if num_ranges_open > 0 && range_start.is_none() {
            range_start = Some(id);
            debug!("At id {}: starting covered range", id);
        } else if num_ranges_open == 0 && range_start.is_some() {
            let covered = id - range_start.unwrap() + 1;  // Ranges are inclusive.
            total_covered += covered;
            range_start = None;
            debug!("At id {}: ending covered range, covered {}", id, covered);
        }
    }
    (in_ranges, total_covered as i64)
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
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 14);
    }

    #[test]
    fn test_test_b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 345821388687084);
    }
}
