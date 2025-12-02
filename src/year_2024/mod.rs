mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
//mod day16;
mod day17;

type SolutionFn = fn(Option<String>) -> i64;
const NAME_TO_FN : &[(&str, SolutionFn)] = &[
    ("day1a", day1::day1),
    ("day1b", day1::day1b),
    ("day2a", day2::day2),
    ("day2b", day2::day2b),
    ("day3a", day3::day3),
    ("day3b", day3::day3b),
    ("day4a", day4::day4),
    ("day4b", day4::day4b),
    ("day5a", day5::day5),
    ("day5b", day5::day5b),
    ("day6a", day6::day6),
    ("day6b", day6::day6b),
    ("day7a", day7::day7),
    ("day7b", day7::day7b),
    ("day8a", day8::day8),
    ("day8b", day8::day8b),
    ("day9a", day9::day9),
    ("day9b", day9::day9b),
    ("day10a", day10::day10),
    ("day10b", day10::day10b),
    ("day11a", day11::day11),
    ("day11b", day11::day11b),
    ("day12a", day12::day12),
    ("day12b", day12::day12b),
    ("day13a", day13::day13),
    ("day13b", day13::day13b),
    ("day14a", day14::day14),
    ("day14b", day14::day14b),
    ("day15a", day15::day15),
    ("day15b", day15::day15b),
    //("day16a", day16::day16),
    //("day16b", day16::day16b),
    // ("day17a", day17::day17),
    ("day17b", day17::day17b),
];

pub fn run_solution(name: &str, input: Option<String>) -> i64 {
    let solutions: std::collections::HashMap<_, _> = NAME_TO_FN.iter().cloned().collect();
    solutions[name](input)
}
