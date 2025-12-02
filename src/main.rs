mod common;
mod year_2025;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value="2025")]
    year: usize,

    #[arg(short, long)]
    day: String,

    source: Option<String>,
}


const YEAR_TO_FN: &[(usize, for<'a> fn(&'a str, Option<std::string::String>) -> i64); 1] = &[
    (2025, year_2025::run_test),
];


fn main() {
    env_logger::init();
    let args = Args::parse();
    let solutions: std::collections::HashMap<_, _> = YEAR_TO_FN.iter().cloned().collect();
    let result: i64 = solutions[&args.year](&args.day, args.source);
    println!("{}", result)
}
