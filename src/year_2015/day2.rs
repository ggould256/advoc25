use log::debug;
use regex::Regex;

use crate::common::parsing::read_regex_records;

#[derive(Debug)]
struct Package {
    length: i64,
    width: i64,
    height: i64,
}

fn read_input(source: Option<String>) -> Vec<Package> {
    let package_re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    let packages = read_regex_records(source, package_re);
    let mut result = vec!();
    for record in packages {
        let length: i64 = record[1].parse().unwrap();
        let width: i64 = record[2].parse().unwrap();
        let height: i64 = record[3].parse().unwrap();
        result.push(Package {
            length,
            width,
            height,
        });
    }
    result
}

pub fn day1(source: Option<String>) -> (i64, i64) {
    let mut total_area = 0;
    let mut total_ribbon = 0;
    for package in read_input(source) {
        let faces = vec![
            package.length * package.width,
            package.width * package.height,
            package.height * package.length,
            package.length * package.width,
            package.width * package.height,
            package.height * package.length,
        ];
        let smallest_face = faces.iter().min().unwrap();
        debug!("Faces: {:?} + {}", faces, smallest_face);
        let wrapping_area = faces.iter().sum::<i64>() + smallest_face;
        total_area += wrapping_area;
        let perimeters = vec![
            2 * (package.length + package.width),
            2 * (package.width + package.height),
            2 * (package.height + package.length),
        ];
        let volume = package.length * package.width * package.height;
        let smallest_perimeter = perimeters.iter().min().unwrap();
        debug!("Perimeters: {:?} + volume: {}", perimeters, volume);
        let ribbon_length = smallest_perimeter + volume;
        total_ribbon += ribbon_length;
    }
    (total_area, total_ribbon)
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

    const DAY: &str = "2";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2015/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2015/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");

    #[test]
    fn test_example_1() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 101);
    }

    #[test]
    fn test_test_1() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 1586300);
    }

    #[test]
    fn test_example_1b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 48);
    }

    #[test]
    fn test_test_1b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 3737498);
    }
}
