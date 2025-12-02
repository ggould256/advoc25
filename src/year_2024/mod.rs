mod day1;
mod day2;
// TODO(ggould) ... rest of the days.

type SolutionFn = fn(Option<String>) -> i64;
const NAME_TO_FN : &[(&str, SolutionFn)] = &[
    ("day1a", day1::day1),
    ("day1b", day1::day1b),
    ("day2a", day2::day2),
    ("day2b", day2::day2b),
    // TODO(ggould) ... rest of the days.
];

pub fn run_solution(name: &str, input: Option<String>) -> i64 {
    let solutions: std::collections::HashMap<_, _> = NAME_TO_FN.iter().cloned().collect();
    solutions[name](input)
}
