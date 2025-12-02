use crate::parsing::read_lines;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}
const CHEAP_OPERATORS: [Operator; 2] = [Operator::Add, Operator::Mul];
const ALL_OPERATORS: [Operator; 3] = [Operator::Add, Operator::Mul, Operator::Concat];

impl Operator {
    pub fn apply(&self, l: i64, r: i64) -> i64 {
        match self {
            Operator::Add => l + r,
            Operator::Mul => l * r,
            Operator::Concat => (l.to_string() + &r.to_string()).parse::<i64>().unwrap(),
        }
    }

    pub fn all_seqs(len: usize, allow_expensive: bool) -> Vec<Vec<Operator>> {
        if len == 0 {
            vec![Vec::new()]
        } else {
            let mut seqs = Vec::new();
            let shorter = Self::all_seqs(len - 1, allow_expensive);
            for shorter_seq in shorter {
                let all_operators = if allow_expensive {
                    Vec::from(ALL_OPERATORS)
                } else {
                    Vec::from(CHEAP_OPERATORS)
                };
                for op in all_operators {
                    let mut new_seq = shorter_seq.clone();
                    new_seq.push(op);
                    seqs.push(new_seq);
                }
            }
            seqs
        }
    }
}

fn parse_line(line: String) -> (i64, Vec<i64>) {
    let sides: Vec<&str> = line.split(":").collect();
    assert!(sides.len() == 2);
    let target = str::parse::<i64>(sides[0]).unwrap();
    let operands: Vec<i64> = sides[1]
        .trim_start()
        .split(" ")
        .map(|s| str::parse::<i64>(s).unwrap())
        .collect();
    (target, operands)
}

pub fn day7_base(source: Option<String>, allow_concat: bool) -> i64 {
    let lines = read_lines(source);
    let mut result: i64 = 0;
    for line in lines {
        let (target, operands) = parse_line(line);
        println!("On {} {:?} ...", target, operands);
        for seq in Operator::all_seqs(operands.len() - 1, allow_concat) {
            let mut operands_iter = operands.iter();
            let mut accumulator = *operands_iter.next().unwrap();
            for op in seq.clone() {
                accumulator = op.apply(accumulator, *operands_iter.next().unwrap());
            }
            if accumulator == target {
                result += target;
                break;
            }
        }
    }
    result
}

pub fn day7(source: Option<String>) -> i64 {
    day7_base(source, false)
}

pub fn day7b(source: Option<String>) -> i64 {
    day7_base(source, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(day7(Some("data/day7_example.txt".to_string())), 3749);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day7(Some("inputs/day7_test.txt".to_string())), 945512582195);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(day7b(Some("data/day7_example.txt".to_string())), 11387);
    }

    // This test is not run as it requires a lot of time.
    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(
            day7b(Some("inputs/day7_test.txt".to_string())),
            271691107779347
        );
    }
}
