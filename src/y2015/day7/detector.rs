use std::collections::HashSet;

use regex::Regex;

use super::operations::*;

pub fn resolve_instruction(line: &String) -> Instruction {
    let init_r = Regex::new(r"^(\d+) -> (\w+)").unwrap();
    let assign_r = Regex::new(r"^(\w+) -> (\w+)").unwrap();
    // let mut initialization_instr: Vec<(u16, String)> = Vec::new();

    let not_r = Regex::new(r"^NOT (\w+) -> (\w+)").unwrap();
    // let mut not_instr: Vec<(String, String)> = Vec::new();

    let shift_r = Regex::new(r"^(\w+) ([LR]SHIFT) (\d+) -> (\w+)").unwrap();
    // let mut shift_instr: Vec<(String, String, u16, String)> = Vec::new();

    let op_r = Regex::new(r"^(\w+) (OR|AND) (\w+) -> (\w+)").unwrap();
    // let mut op_instr: Vec<(String, String, String, String)> = Vec::new();

    let instruction: Instruction;

    if let Some(captured) = init_r.captures(line) {
        let num = captured[1].parse::<u16>().unwrap();
        let wire = captured[2].parse::<String>().unwrap();

        instruction = Instruction::Init(OInit { num, wire });
    } else if let Some(captured) = assign_r.captures(line) {
        let wire_in = captured[1].parse::<String>().unwrap();
        let wire_out = captured[2].parse::<String>().unwrap();
        let mut deps = HashSet::new();
        deps.insert(wire_in.clone());

        instruction = Instruction::Assign(OAssign {
            wire_in,
            deps,
            wire_out,
        });
    } else if let Some(captured) = not_r.captures(line) {
        let wire_in = captured[1].parse::<String>().unwrap();
        let wire_out = captured[2].parse::<String>().unwrap();
        let mut deps = HashSet::new();
        deps.insert(wire_in.clone());

        instruction = Instruction::Not(ONot {
            wire_in,
            deps,
            wire_out,
        });
    } else if let Some(captured) = shift_r.captures(line) {
        let wire_in = captured[1].parse::<String>().unwrap();
        let operation = captured[2].parse::<String>().unwrap();
        let num = captured[3].parse::<u16>().unwrap();
        let wire_out = captured[4].parse::<String>().unwrap();
        let mut deps = HashSet::new();
        deps.insert(wire_in.clone());

        instruction = Instruction::Shift(OShift {
            wire_in,
            deps,
            operation,
            num,
            wire_out,
        });
    } else if let Some(captured) = op_r.captures(line) {
        let wire_a = captured[1].parse::<String>().unwrap();
        let operation = captured[2].parse::<String>().unwrap();
        let wire_b = captured[3].parse::<String>().unwrap();
        let wire_out = captured[4].parse::<String>().unwrap();
        let mut deps = HashSet::new();
        if wire_a != "1" {
            deps.insert(wire_a.clone());
        }
        deps.insert(wire_b.clone());

        instruction = Instruction::Logic(OLogic {
            wire_a,
            operation,
            wire_b,
            deps,
            wire_out,
        });
    } else {
        eprintln!("Line: {line} didn't match any regex...");
        todo!();
    }

    return instruction;
}
