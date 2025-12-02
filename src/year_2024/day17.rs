use regex::Regex;


type RegVal = u32;
type ThreeBit = u8;
type Program = Vec<ThreeBit>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Iadv,
    Ibxl,
    Ibst,
    Ijnz,
    Ibxc,
    Iout,
    Ibdv,
    Icdv,
}
use Instruction::*;
const INSTRUCTIONS: [Instruction; 8] = [Iadv, Ibxl, Ibst, Ijnz, Ibxc, Iout, Ibdv, Icdv,];

#[derive(Debug, Clone)]
struct VmState {
    registers: [RegVal; 3],
    instruction_pointer: usize,
    program: Program,
    output_so_far: Vec<RegVal>,
    halted: bool,
}

impl VmState {
    fn from_record(record: &Vec<String>) -> VmState {
        let a: RegVal = record[1].parse().unwrap();
        let b: RegVal = record[2].parse().unwrap();
        let c: RegVal = record[3].parse().unwrap();
        let program = record[4].split(",").map(|i| i.parse().unwrap()).collect();
        VmState{registers: [a, b, c],
                instruction_pointer: 0,
                program,
                output_so_far: Vec::new(),
                halted: false}
    }

    fn resolve_combo(&self, operand: ThreeBit) -> Option<RegVal> {
        let result = {
            if operand <= 3 {
                Some(operand as RegVal)
            } else if operand < 7 {
                Some(self.registers[(operand - 4) as usize])
            } else {
                None
            }
        };
        result
    }

    fn do_dv(&self, pow: RegVal) -> RegVal{
        (self.registers[0] as f64 / f64::powi(2.0, pow as i32)) as RegVal
    }

    fn advance_state(&self) -> VmState {
        let mut new_state = self.clone();
        if self.halted { return new_state; }
        let instruction = INSTRUCTIONS[self.program[self.instruction_pointer] as usize];
        let operand = self.program[self.instruction_pointer + 1 as usize];
        let mut jumped = false;
        match instruction {
            Iadv => {
                if let Some(pow) = new_state.resolve_combo(operand) {
                    new_state.registers[0] = new_state.do_dv(pow);
                } else { new_state.halted = true; }
            },
            Ibxl => {
                new_state.registers[1] = new_state.registers[1] ^ (operand as RegVal);
            },
            Ibst => {
                if let Some(new) = new_state.resolve_combo(operand) {
                    new_state.registers[1] = new % 8;
                } else { new_state.halted = true; }
            },
            Ijnz => {
                if new_state.registers[0] != 0 {
                    jumped = true;
                    new_state.instruction_pointer = operand as usize;
                }
            },
            Ibxc => {
                new_state.registers[1] ^= new_state.registers[2];

            },
            Iout => {
                if let Some(out) = new_state.resolve_combo(operand) {
                    new_state.output_so_far.push(out % 8);
                } else { new_state.halted = true; }
            },
            Ibdv => {
                if let Some(pow) = new_state.resolve_combo(operand) {
                    new_state.registers[1] = self.do_dv(pow);
                } else { new_state.halted = true; }
            },
            Icdv => {
                if let Some(pow) = new_state.resolve_combo(operand) {
                    new_state.registers[2] = self.do_dv(pow);
                } else { new_state.halted = true; }
            },
        };
        if !jumped { new_state.instruction_pointer += 2; }
        if new_state.instruction_pointer >= new_state.program.len() {
            new_state.halted = true;
        }
        new_state
    }

    fn run_until_halt(&self, max_output_len: usize) -> Vec<RegVal> {
        let mut vm_state = self.clone();
        while !vm_state.halted && vm_state.output_so_far.len() < max_output_len {
            vm_state = vm_state.advance_state();
        }    
        vm_state.output_so_far
    }

    fn expect_output(&self, expected_output: &Vec<RegVal>) -> bool {
        let mut vm_state = self.clone();
        while !vm_state.halted {
            vm_state = vm_state.advance_state();
            if !expected_output.starts_with(&vm_state.output_so_far) {
                return false;
            }
        }
        true
    }
}



pub fn day17(source: Option<String>) -> String {
    let records = crate::parsing::read_regex_records(
        source,
        Regex::new(concat!(
            r"Register A: (\d+)\n+",
            r"Register B: (\d+)\n+",
            r"Register C: (\d+)\n+",
            r"Program: ([\d,]+)\n+")).unwrap()
    );
    assert!(records.len() == 1);
    let record: &Vec<String> = &records[0];
    let vm_state = VmState::from_record(record);
    let output = vm_state.run_until_halt(100);
    let output_strings: Vec<String> = output.iter().map(|v| v.to_string()).collect();
    output_strings.join(",")
}

pub fn day17b(source: Option<String>) -> i64 {
    let records = crate::parsing::read_regex_records(
        source,
        Regex::new(concat!(
            r"Register A: (\d+)\n+",
            r"Register B: (\d+)\n+",
            r"Register C: (\d+)\n+",
            r"Program: ([\d,]+)\n+")).unwrap()
    );
    assert!(records.len() == 1);
    let record: &Vec<String> = &records[0];
    let vm_state = VmState::from_record(record);
    let mut candidate: RegVal = 0;
    let program_as_regval: Vec<RegVal> = vm_state.program.iter().map(|r| *r as RegVal).collect();
    loop {
        let mut new_vm = vm_state.clone();
        new_vm.registers[0] = candidate;
        if vm_state.expect_output(&program_as_regval) {
            return candidate as i64;
        }
        if candidate & 100000 == 0 {
            println!("Checking candidate value {}: {:?}", candidate, vm_state.run_until_halt(16))
        }
        candidate += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        assert_eq!(day17(Some("data/day17_example.txt".to_string())), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test() {
        assert_eq!(day17(Some("inputs/day17_test.txt".to_string())), "");
    }

    #[test]
    #[ignore = "TODO"]
    fn test_example_b() {
        assert_eq!(day17b(Some("data/day17_example_b.txt".to_string())), 117440);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_b() {
        assert_eq!(day17b(Some("inputs/day17_test.txt".to_string())), 0);
    }

    // B cannot be tested.
}
