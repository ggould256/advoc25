mod day1;

const NAME_TO_FN : &[(&str, fn(Option<String>) -> i64)] = &[
    ("day1a", day1::solution_a),
    ("day1b", day1::solution_b),
];

pub fn run_test(name: &str, input: Option<String>) -> i64 {
    let solutions: std::collections::HashMap<_, _> = NAME_TO_FN.iter().cloned().collect();
    solutions[name](input)
}
