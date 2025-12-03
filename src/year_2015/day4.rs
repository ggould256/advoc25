use crate::common::parsing::read_one_string;

fn read_input(source: Option<String>) -> String {
    
    read_one_string(source)
}

pub fn solution(source: Option<String>, zeroes: usize) -> i64 {
    let prefix = read_input(source);
    for i in 0.. {
        let test_string = format!("{}{}", prefix, i);
        let digest = md5::compute(test_string.as_bytes());
        let digest_as_hex = format!("{:x}", digest);
        if digest_as_hex.starts_with(&"0".repeat(zeroes)) {
            return i;
        }
    }
    unreachable!()
}

pub fn solution_a(source: Option<String>) -> i64 {
    solution(source, 5)
}

pub fn solution_b(source: Option<String>) -> i64 {
    solution(source, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "4";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2015/day", DAY, "_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2015/day", DAY, "_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");

    #[test]
    fn test_example_a() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 609043);
    }

    #[test]
    fn test_test_a() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 282749);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 6742839);
    }

    #[test]
    fn test_test_b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 9962624);
    }
}
