use std::collections::HashMap;

use crate::parsing::read_all_records;

type Stone = u64;

type StoneMultiset = HashMap<Stone, usize>;

pub fn count_stones(stones: &StoneMultiset) -> usize {
    stones.values().sum()
}

pub fn update_stone(stone: Stone) -> Vec<Stone> {
    let stone_str = format!("{}", stone);
    if stone == 0 {
        vec![1]
    } else if stone_str.len() % 2 == 0 {
        let old_len: usize = stone_str.len();
        let new_len: usize = stone_str.len() / 2;
        vec![stone_str[0..new_len].parse::<Stone>().unwrap(),
             stone_str[new_len..old_len].parse::<Stone>().unwrap()]
    } else {
        vec![stone * 2024]
    }
}

pub fn do_blinks(source: Option<String>, num_iterations: usize) -> i64 {
    let starting_configuration = read_all_records(source);
    let mut starting_stones = StoneMultiset::new();
    
    for stone_str in &starting_configuration[0] {
        let stone: Stone = stone_str.parse().unwrap();
        *starting_stones.entry(stone).or_default() += 1;
    }
    let starting_stones = starting_stones;
    let mut working_stones: StoneMultiset = starting_stones.clone();

    println!("Starting stones are: {} stones {:?}", count_stones(&working_stones), working_stones);        
    for i in 0..num_iterations {
        let mut new_working: HashMap<Stone, usize> = HashMap::new();
        for (stone, count) in working_stones.iter() {
            for new_stone in update_stone(*stone) {
                *new_working.entry(new_stone).or_default() += count;
            }
        }
        working_stones = new_working;
        println!("After {} iterations: {} stones {:?}", i, count_stones(&working_stones), working_stones);        
    }
    count_stones(&working_stones).try_into().unwrap()
}

pub fn day11(source: Option<String>) -> i64 {
    do_blinks(source, 25)
}

pub fn day11b(source: Option<String>) -> i64 {
    do_blinks(source, 75)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day11(Some("data/day11_example.txt".to_string())), 55312);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day11(Some("inputs/day11_test.txt".to_string())), 207683);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(day11b(Some("inputs/day11_test.txt".to_string())), 244782991106220);
    }
}
