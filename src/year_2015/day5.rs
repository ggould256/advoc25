use crate::common::parsing::read_lines;

use fancy_regex::Regex;

fn accept_string_a(s: &str) -> bool {
    let three_vowels_re = Regex::new(r".*[aeiou].*[aeiou].*[aeiou].*").unwrap();
    let double_letter_re = Regex::new(r"(.)\1").unwrap();
    let bad_combo_re = Regex::new(r".*(ab|cd|pq|xy).*").unwrap();
    three_vowels_re.is_match(s).unwrap()
        && double_letter_re.is_match(s).unwrap()
        && !bad_combo_re.is_match(s).unwrap()
}

fn accept_string_b(s: &str) -> bool {
    let doubled_pair_re = Regex::new(r".*(..).*\1.*").unwrap();
    let bracketed_letter_re = Regex::new(r".*(.).\1.*").unwrap();

    doubled_pair_re.is_match(s).unwrap() && bracketed_letter_re.is_match(s).unwrap()
}

fn solution(source: Option<String>) -> (i64, i64) {
    let lines = read_lines(source);
    let accepted_lines_a = lines.iter().filter(|s| accept_string_a(s)).count();
    let accepted_lines_b = lines.iter().filter(|s| accept_string_b(s)).count();
    (accepted_lines_a as i64, accepted_lines_b as i64)
}

pub fn solution_a(source: Option<String>) -> i64 {
    solution(source).0
}

pub fn solution_b(source: Option<String>) -> i64 {
    solution(source).1
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "5";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2015/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2015/day", DAY, "b_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");

    #[test]
    fn test_example_a() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 2);
    }

    #[test]
    fn test_test_a() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 258);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 2);
    }

    #[test]
    fn test_test_b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 53);
    }
}
