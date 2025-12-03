use log::debug;

use crate::common::parsing::read_lines;

type Battery = Vec<i64>;

fn read_input(source: Option<String>) -> Vec<Battery> {
    let lines = read_lines(source);
    let mut batteries: Vec<Battery> = Vec::new();
    for line in lines {
        let levels: Battery = line
            .chars()
            .map(|c| c.to_string().parse::<i64>().unwrap().to_owned())
            .collect::<Vec<i64>>();
        batteries.push(levels);
    }
    batteries
}

fn compute_power(batteries: &Vec<Battery>, num_cells: usize) -> i64 {
    let mut total_power: i64 = 0;
    for battery in batteries {
        debug!("On battery {:?} choosing {} cells:", battery, num_cells);
        let mut battery_power = 0;
        let mut cursor = 0;
        for cell in 0..num_cells {
            let cells_remaining = num_cells - cell;
            debug!(
                "battery has {} cells, {} cells remain to be chosen, cursor at {}",
                battery.len(),
                cells_remaining,
                cursor
            );
            assert!(cursor + cells_remaining <= battery.len());
            let digit = battery
                .iter()
                .skip(cursor)
                .take(battery.len() - cursor - cells_remaining + 1)
                .max()
                .unwrap();
            let digit_pos = battery
                .iter()
                .skip(cursor)
                .position(|&x| x == *digit)
                .unwrap()
                + cursor;
            cursor = digit_pos + 1;
            debug!("Selected digit at pos {} with value {}", digit_pos, digit);
            battery_power = battery_power * 10 + digit;
        }
        total_power += battery_power;
    }
    total_power
}

pub fn day3(source: Option<String>) -> (i64, i64) {
    let records = read_input(source);
    (compute_power(&records, 2), compute_power(&records, 12))
}

pub fn solution_a(source: Option<String>) -> i64 {
    day3(source).0 as i64
}

pub fn solution_b(source: Option<String>) -> i64 {
    day3(source).1 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "3";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");

    #[test]
    fn test_example_1() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 357);
    }

    #[test]
    fn test_test_1() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 17092);
    }

    #[test]
    fn test_example_1b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 3121910778619);
    }

    #[test]
    fn test_test_1b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 170147128753455);
    }
}
