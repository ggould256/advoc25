use regex::Regex;

use log::debug;

use crate::common::parsing::read_regex_records;

#[derive(Debug)]
enum Lr {
    Left,
    Right,
}

#[derive(Debug)]
struct Action {
    direction: Lr,
    distance: i32,
}

fn read_input(source: Option<String>) -> Vec<Action> {
    let regex = Regex::new(r"([LR])(\d+)");
    let records = read_regex_records(source, regex.unwrap());
    let mut result: Vec<Action> = vec![];
    for record in records {
        assert_eq!(record.len(), 3); // Full match, direction, distance
        let action = Action {
            direction: match record[1].as_str() {
                "L" => Lr::Left,
                "R" => Lr::Right,
                _ => {
                    panic!("Parsing failure")
                }
            },
            distance: record[2].parse::<i32>().unwrap(),
        };
        result.push(action);
    }
    result
}

const INITIAL_POSITION: i32 = 50;
const DIAL_SIZE: i32 = 100;

fn count_clicks(from_pos: i32, offset: i32) -> (i32, i32) {
    let mut zero_arrivals = 0;

    // A full rotation of the dial will generate a click.
    let full_rotations = offset.abs() / DIAL_SIZE;
    zero_arrivals += full_rotations;

    // Net of full rotations, what is the remaining offset?
    let remaining_offset = offset % DIAL_SIZE; // SIGNED in rust.

    // Does that remaining offset cross zero?
    let new_signed_pos = from_pos + remaining_offset;
    debug!(
        "From pos {} offset {} new pos {}",
        from_pos, remaining_offset, new_signed_pos
    );
    #[allow(clippy::nonminimal_bool)] // For clarity.
    if (from_pos > 0 && new_signed_pos <= 0) || (from_pos > 0 && new_signed_pos >= DIAL_SIZE) {
        debug!("Crossed zero");
        zero_arrivals += 1;
    }

    let end_at_zero = (from_pos + offset) % DIAL_SIZE == 0;
    (end_at_zero as i32, zero_arrivals)
}

pub fn day1(source: Option<String>) -> (i32, i32) {
    let records = read_input(source);
    let mut position = INITIAL_POSITION;
    let mut zero_visits = 0;
    let mut zero_passes = 0;
    for record in records {
        debug!("Record: {:?} ", record);
        match record.direction {
            Lr::Left => {
                let (zero_visits_inc, zero_passes_inc) = count_clicks(position, -record.distance);
                zero_visits += zero_visits_inc;
                zero_passes += zero_passes_inc;
                position = (position - record.distance).rem_euclid(DIAL_SIZE);
            }
            Lr::Right => {
                let (zero_visits_inc, zero_passes_inc) = count_clicks(position, record.distance);
                zero_visits += zero_visits_inc;
                zero_passes += zero_passes_inc;
                position = (position + record.distance).rem_euclid(DIAL_SIZE);
            }
        }
        debug!("Position: {}", position);
    }
    (zero_visits, zero_passes)
}

pub fn solution_a(source: Option<String>) -> i64 {
    day1(source).0 as i64
}

pub fn solution_b(source: Option<String>) -> i64 {
    day1(source).1 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "1";
    const EXAMPLE_A_DATA: &str = concatcp!("data/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/day", DAY, "a_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/day", DAY, "a_test.txt");

    #[test]
    fn test_example_1() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 3);
    }

    #[test]
    fn test_test_1() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 1040);
    }

    #[test]
    fn test_example_1b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 6);
    }

    #[test]
    fn test_test_1b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 6027);
    }
}
