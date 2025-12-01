use std::env;

use const_format::concatcp;

mod parsing;

macro_rules! register_days {
    ($($module:tt),*) => (
        $(
            mod $module;
        )*
        const NAME_TO_FN : &[(&str, fn(Option<String>) -> i64)] = &[
            $(
                (concatcp!(stringify!($module), "a"), $module::day1a as fn(Option<String>) -> i64),
                (concatcp!(stringify!($module), "b"), $module::day1b as fn(Option<String>) -> i64),
            )*
        ];
    );
}
register_days!(day1);

fn main() {
    let mut failed = false;
    let solutions: std::collections::HashMap<_, _> = NAME_TO_FN.iter().cloned().collect();
    for arg in env::args().skip(1) {
        if solutions.contains_key(arg.as_str()) {
            let func = solutions[&arg.as_str()];
            let result = func(None);
            println!("{} output: {}", arg, result)
        } else {
            println!("Skipping unknown test {}", arg);
            failed = true;
            continue;
        };
    }
    if failed {
        std::process::exit(1);
    }
}
