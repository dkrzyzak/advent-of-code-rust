use crate::parse_input;

use instructions::*;
mod instructions;

pub fn task() {
    let lines = parse_input!();
    let instructions = extract_instructions(&lines);

    let mut index: isize = 0;
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    while index >= 0 && index < instructions.len() as isize {
        let instr = &instructions[index as usize];
        println!("Parsing instruction: {:?} from index {index}", instr);

        match instr {
            Instruction::Increment(register) => {
                if *register == Register::A {
                    a += 1;
                } else {
                    b += 1;
                }

                index += 1;
            }
            Instruction::Half(register) => {
                if *register == Register::A {
                    a /= 2;
                } else {
                    b /= 2;
                }

                index += 1;
            }
            Instruction::Triple(register) => {
                if *register == Register::A {
                    // println!("A: {a}");
                    a *= 3;
                } else {
                    b *= 3;
                }

                index += 1;
            }

            Instruction::Jump(offset) => {
                index += *offset as isize;
            }

            Instruction::JumpIfEven(register, offset) => {
                let cmp_value = if *register == Register::A { a } else { b };
                if cmp_value % 2 == 0 {
                    index += *offset as isize;
                } else {
                    index += 1;
                }
            }

            Instruction::JumpIfOne(register, offset) => {
                let cmp_value = if *register == Register::A { a } else { b };
                if cmp_value == 1 {
                    index += *offset as isize;
                } else {
                    index += 1;
                }
            }
        }
    }

    println!("value in b: {b}");
}
