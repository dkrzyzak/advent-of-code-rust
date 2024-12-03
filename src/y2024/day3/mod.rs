use regex::Regex;

use crate::parse_input;


#[derive(Debug)]
enum Instruction {
    Multiply(usize, usize),
    Enable,
    Disable,
}

pub fn task() {
    let lines = parse_input!();

    // the entry data is split over multiple lines, 
    // so we need to keep the global counter as well as global "enabled" state
    // and update it after processing each another line
    let mut total = 0usize;
    let mut enabled = true;

    let mul_reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let enable_reg = Regex::new(r"do\(\)").unwrap();
    let disable_reg = Regex::new(r"don't\(\)").unwrap();
    
    for line in lines.iter() {
        let mut instructions: Vec<(usize, Instruction)> = Vec::new();

        // parse mul(X,Y) instructions 
        for captures in mul_reg.captures_iter(line) {
            let a_match = captures.get(1).unwrap();
            let a_index = a_match.start();
        
            let a = a_match.as_str().parse::<usize>().unwrap();
            let b = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

            instructions.push((a_index, Instruction::Multiply(a, b)));
        }

        // parse do() instructions 
        for matched in enable_reg.find_iter(line) {
            instructions.push((matched.start(), Instruction::Enable));
        }

        // parse don't() instructions 
        for matched in disable_reg.find_iter(line) {
            instructions.push((matched.start(), Instruction::Disable));
        }

        instructions.sort_by_key(|(index, _)| *index);

        let (parsed_line, new_enabled) = parse_instructions(&instructions, enabled);

        total += parsed_line;
        enabled = new_enabled;

    }

    println!("Total: {}", total);
}

fn parse_instructions(instructions: &Vec<(usize, Instruction)>, mut enabled: bool) -> (usize, bool) {
    let mut total = 0usize;
    
    for (_, instr) in instructions.iter() {
        match *instr {
            Instruction::Enable => { enabled = true },
            Instruction::Disable => { enabled = false },
            Instruction::Multiply(a, b) => {
                if enabled {
                    total += a * b;
                }
            }

        }
    }

    (total, enabled)
}