use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Register {
    A,
    B,
}

#[derive(Debug)]
pub enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i16),
    JumpIfOne(Register, i16),
    JumpIfEven(Register, i16),
}

pub fn extract_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<Instruction>>()
}

fn parse_line(line: &String) -> Instruction {
    let simple_reg = Regex::new(r"(\w+) (a|b)$").unwrap();
    if let Some(captured) = simple_reg.captures(line) {
        let instr_name = captured[1].parse::<String>().unwrap();

        let register = if captured[2].parse::<String>().unwrap() == "a" {
            Register::A
        } else {
            Register::B
        };

        match instr_name.as_str() {
            "hlf" => {
                return Instruction::Half(register);
            }
            "tpl" => {
                return Instruction::Triple(register);
            }
            "inc" => {
                return Instruction::Increment(register);
            }
            _ => {}
        }
    }

    let cond_jump_reg = Regex::new(r"^(\w+) (a|b), ([\+\-]\d+)").unwrap();
    if let Some(captured) = cond_jump_reg.captures(line) {
        let instr_name = captured[1].parse::<String>().unwrap();
        let register = if captured[2].parse::<String>().unwrap() == "a" {
            Register::A
        } else {
            Register::B
        };
        let offset = captured[3].parse::<i16>().unwrap();

        if instr_name == "jie" {
            return Instruction::JumpIfEven(register, offset);
        } else {
            return Instruction::JumpIfOne(register, offset);
        }
    }

    let jump_reg = Regex::new(r"jmp ([\+\-]\d+)").unwrap();
    if let Some(captured) = jump_reg.captures(line) {
        let offset = captured[1].parse::<i16>().unwrap();
        return Instruction::Jump(offset);
    }

    panic!("Line didn't meet criteria for any type of instruction");
}
