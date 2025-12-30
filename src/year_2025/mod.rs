mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

type SolutionFn = fn(Option<String>) -> i64;
const NAME_TO_FN : &[(&str, SolutionFn)] = &[
    ("day1a", day1::solution_a),
    ("day1b", day1::solution_b),
    ("day2a", day2::solution_a),
    ("day2b", day2::solution_b),
    ("day3a", day3::solution_a),
    ("day3b", day3::solution_b),
    ("day4a", day4::solution_a),
    ("day4b", day4::solution_b),
    ("day5a", day5::solution_a),
    ("day5b", day5::solution_b),
    ("day6a", day6::solution_a),
    ("day6b", day6::solution_b),
    ("day7a", day7::solution_a),
    ("day7b", day7::solution_b),
];

pub fn run_solution(name: &str, input: Option<String>) -> i64 {
    let solutions: std::collections::HashMap<_, _> = NAME_TO_FN.iter().cloned().collect();
    solutions[name](input)
}
