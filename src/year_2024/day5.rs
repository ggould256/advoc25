use crate::parsing::read_lines;

fn read_rules(lines: &Vec<String>) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    for line in lines {
        if line.contains("|") {
            let rule_elements: Vec<u32> = line
                .split("|")
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect();
            assert!(rule_elements.len() == 2);
            result.push((rule_elements[0], rule_elements[1]));
        }
    }
    result
}

fn read_updates(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    for line in lines {
        if line.contains(",") {
            let update: Vec<u32> = line
                .split(",")
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect();
            result.push(update);
        }
    }
    result
}

fn valid_update(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for (low, high) in rules {
        let mut seen_high = false;
        for number in update {
            seen_high |= number == high;
            if number == low && seen_high {
                return false;
            }
        }
    }
    println!("  Okay.");
    true
}

fn fix_update(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    println!("considering update {:?}:", update);
    let mut result = update.clone();
    let mut did_swap = true;
    while did_swap {
        did_swap = false;
        for (low, high) in rules {
            let mut seen_high = false;
            let mut high_idx: usize = 0;
            for i in 0..update.len() {
                let number = &result[i];
                if number == high {
                    seen_high = true;
                    high_idx = i;
                } else if number == low && seen_high {
                    println!("  violated rule {}|{}; swapping.", low, high);
                    did_swap = true;
                    result[high_idx] = *low;
                    result[i] = *high;
                }
            }
        }
    }
    println!("  Update is now {:?}:", result);
    println!("  Okay.");
    result
}

fn middle_element(update: &[u32]) -> u32 {
    update[(update.len() - 1) / 2]
}

pub fn day5(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    let rules = read_rules(&lines);
    let updates = read_updates(&lines);
    let valid_updates = updates.iter().filter(|&u| valid_update(u, &rules));
    let total: u32 = valid_updates.map(|u| middle_element(u)).sum();
    i32::try_from(total).unwrap()
}

pub fn day5b(source: Option<String>) -> i32 {
    let lines = read_lines(source);
    let rules = read_rules(&lines);
    let updates = read_updates(&lines);
    let invalid_updates = updates.iter().filter(|&u| !valid_update(u, &rules));
    let newly_valid_updates = invalid_updates.map(|u| fix_update(u, &rules));
    let total: u32 = newly_valid_updates.map(|u| middle_element(&u)).sum();
    i32::try_from(total).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day5(Some("data/day5_example.txt".to_string())), 143);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day5(Some("inputs/day5_test.txt".to_string())), 6242);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day5b(Some("data/day5_example.txt".to_string())), 123);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(day5b(Some("inputs/day5_test.txt".to_string())), 5169);
    }
}
