use crate::parsing::{parse_as_i32s, read_all_records};

fn record_is_safe(record: &[i32]) -> bool {
    let differences: Vec<i32> = (0..record.len() - 1)
        .map(|i| record[i + 1] - record[i])
        .collect();
    // Ascending or descending.
    if differences.iter().all(|&diff| diff > 0) || differences.iter().all(|&diff| diff < 0) {
        // Bounded.
        if differences.iter().all(|&diff| (-3..=3).contains(&diff)) {
            return true;
        }
    }
    false
}

fn record_is_kinda_safe(record: &[i32]) -> bool {
    let mut possible_safe_sequences: Vec<Vec<i32>> = vec![record.to_owned()];
    for i in 0..record.len() {
        let mut short_record = Vec::new();
        short_record.extend(record[0..i].iter());
        short_record.extend(record[i + 1..record.len()].iter());
        possible_safe_sequences.push(short_record);
    }
    possible_safe_sequences.iter().any(|r| record_is_safe(r))
}

fn total_safe(records: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for record in records {
        if record_is_safe(&record) {
            result += 1;
        }
    }
    result
}

fn total_kinda_safe(records: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for record in records {
        if record_is_kinda_safe(&record) {
            result += 1;
        }
    }
    result
}

pub fn day2(source: Option<String>) -> i32 {
    let records = parse_as_i32s(read_all_records(source));
    total_safe(records)
}

pub fn day2b(source: Option<String>) -> i32 {
    let records = parse_as_i32s(read_all_records(source));
    total_kinda_safe(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day2(Some("data/day2_example.txt".to_string())), 2);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day2(Some("inputs/day2_test.txt".to_string())), 516);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day2b(Some("data/day2_example.txt".to_string())), 4);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(day2b(Some("inputs/day2_test.txt".to_string())), 561);
    }
}
