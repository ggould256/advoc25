use std::collections::{HashMap, HashSet};
use std::iter::zip;
use std::vec::Vec;

use crate::parsing::{parse_as_ii, read_all_records};

fn sorted_error_sum(records: Vec<(i32, i32)>) -> i32 {
    let mut result = 0;
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = records.iter().cloned().unzip();
    left.sort();
    right.sort();
    for (x, y) in zip(left, right) {
        result += (x - y).abs();
    }
    result
}

fn similarity_score(records: Vec<(i32, i32)>) -> i32 {
    let mut result: i32 = 0;
    let (left, right): (Vec<i32>, Vec<i32>) = records.iter().cloned().unzip();
    let keys: HashSet<i32> =
        &HashSet::from_iter(left.iter().cloned()) | &HashSet::from_iter(left.iter().cloned());
    let mut left_counts: HashMap<i32, i32> = HashMap::new();
    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    for l in left {
        left_counts.insert(l, left_counts.get(&l).unwrap_or(&0) + 1);
    }
    for r in right {
        right_counts.insert(r, right_counts.get(&r).unwrap_or(&0) + 1);
    }
    for k in keys {
        let score: i32 = k * left_counts.get(&k).unwrap_or(&0) * right_counts.get(&k).unwrap_or(&0);
        result += score;
    }
    result
}

pub fn day1(source: Option<String>) -> i32 {
    let records = read_all_records(source);
    let parsed = parse_as_ii(records);
    sorted_error_sum(parsed)
}

pub fn day1b(source: Option<String>) -> i32 {
    let records = read_all_records(source);
    let parsed = parse_as_ii(records);
    similarity_score(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(day1(Some("data/day1_example.txt".to_string())), 11);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_1() {
        assert_eq!(day1(Some("inputs/day1_test.txt".to_string())), 1319616);
    }

    #[test]
    fn test_example_1b() {
        assert_eq!(day1b(Some("data/day1_example.txt".to_string())), 31);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_1b() {
        assert_eq!(day1b(Some("inputs/day1_test.txt".to_string())), 27267728);
    }
}
