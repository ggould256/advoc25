use std::{clone, collections::HashMap, fmt::Display};

use log::debug;

use crate::common::parsing::{read_all_records};

type Signal = u16;
type Label = String;

#[derive(Debug)]
enum Op { And, Or, Not, Lshift, Rshift, Nop }
impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "AND" => Op::And,
            "OR" => Op::Or,
            "NOT" => Op::Not,
            "LSHIFT" => Op::Lshift,
            "RSHIFT" => Op::Rshift,
            _ => panic!("Unknown operation {}", s),
        }
    }
}

#[derive(Debug, Clone)]
enum Value { Literal (u16), Label (Label) }

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        if let Ok(lit) = s.parse::<u16>() {
            Value::Literal(lit)
        } else {
            Value::Label(s.to_string())
        }
    }
}

#[derive(Debug)]
struct Node {
    output: Label,
    input: Value,
    op: Op,
    extra_input: Option<Value>,
}

fn read_input(source: Option<String>) -> Vec<Node> {
    let lines = read_all_records(source);
    let mut nodes: Vec<Node> = Vec::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        if line[0] == "NOT" {
            nodes.push(Node{
                output: line[3].to_string(),
                input: Value::from(line[1].as_str()),
                op: Op::Not,
                extra_input: None,
            });
        }
        else if line[1] == "->" {
            nodes.push(Node{
                output: line[2].to_string(),
                input: Value::from(line[0].as_str()),
                op: Op::Nop,
                extra_input: None,
            });
        }
        else {
            nodes.push(Node{
                output: line[2].to_string(),
                input: Value::from(line[0].as_str()),
                op: Op::from(line[1].as_str()),
                extra_input: None,
            });
        }
    }
    nodes
}

/// For any given signal, find the node that outputs that signal.
fn labels_to_sources(nodes: &Vec<Node>) -> HashMap<Label, usize> {
    let mut sources: HashMap<Label, usize> = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        if sources.contains_key(&node.output) {
            panic!("Multiple sources for signal {}", node.output);
        }
        sources.insert(node.output.clone(), i);
    }
    sources
}

fn compute_value(value: &Value, nodes: &Vec<Node>,
                 labels_to_signals: &mut HashMap<Label, Signal>)  -> Signal {
    let mut labels_to_signals = HashMap::<Label, Signal>::new();
    let sources = labels_to_sources(nodes);
    let computed_value: Signal = match value {
        Value::Literal(l) => *l,
        Value::Label(l) => {
            if labels_to_signals.contains_key(l) {
                labels_to_signals[l]
            } else {
                let source_node = &nodes[sources[l]];
                match source_node.op {
                    Op::Nop => compute_value(&source_node.input, nodes, &mut labels_to_signals),
                    Op::Not => !compute_value(&source_node.input, nodes, &mut labels_to_signals),  // Rust `!` is bitwise.
                    Op::And => compute_value(&source_node.input, nodes, &mut labels_to_signals)
                               & compute_value(&source_node.extra_input.clone().unwrap(), nodes, &mut labels_to_signals),
                    Op::Or => compute_value(&source_node.input, nodes, &mut labels_to_signals)
                              | compute_value(&source_node.extra_input.clone().unwrap(), nodes, &mut labels_to_signals),
                    Op::Lshift => compute_value(&source_node.input, nodes, &mut labels_to_signals)
                              << compute_value(&source_node.extra_input.clone().unwrap(), nodes, &mut labels_to_signals),
                    Op::Rshift => compute_value(&source_node.input, nodes, &mut labels_to_signals)
                              >> compute_value(&source_node.extra_input.clone().unwrap(), nodes, &mut labels_to_signals),
                }
            }   
        }
    };
    if let Value::Label(l) = value {
        labels_to_signals.insert(l.clone(), computed_value);
    }
    computed_value
}

fn solution(source: Option<String>) -> (i64, i64) {
    let nodes = read_input(source);


    (0, 0)
}

pub fn solution_a(source: Option<String>) -> i64 {
    solution(source).0
}

pub fn solution_b(source: Option<String>) -> i64 {
    solution(source).1
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "6";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2015/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2015/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");

    #[test]
    fn test_example_a() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 998996);
    }

    #[test]
    fn test_test_a() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 400410);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 1001996);
    }

    #[test]
    fn test_test_b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 15343601);
    }
}
