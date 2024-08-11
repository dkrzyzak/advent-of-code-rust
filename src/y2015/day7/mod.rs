use crate::read::read_input_file;
use std::collections::HashMap;

mod detector;
mod operations;

use operations::*;

pub fn task() {
    let lines = read_input_file("y2015", "day7");

    let mut circuit_map: HashMap<String, u16> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines.iter() {
        let parsed = detector::resolve_instruction(line);

        match &parsed {
            Instruction::Init(init) => {
                circuit_map.insert(init.wire.clone(), init.num);
            }
            _ => {
                instructions.push(parsed);
            }
        }
    }

    println!("Finalized initializing instructions vector");

    let mut known_deps: Vec<String> = vec!["b".to_string(), "c".to_string()];
    circuit_map.insert("1".to_string(), 1);

    while !known_deps.is_empty() {
        let dep = known_deps.pop().unwrap();
        let mut cleared_instructions_indices: Vec<usize> = Vec::new();

        for i in 0..instructions.len() {
            let instr = &mut instructions[i];
            let deps = instr.deps().unwrap();

            if !deps.contains(&dep) {
                continue;
            }

            deps.remove(&dep);

            // if we have all we need to perform the operation
            if deps.is_empty() {
                let wire = instr.wire_out();

                // perform the operation
                match &instr {
                    Instruction::Assign(assign) => {
                        let wire_in = circuit_map
                            .get(&assign.wire_in)
                            .expect("Missing value for assignment...");
                        
                        circuit_map.insert(assign.wire_out.clone(), *wire_in);
                    }
                    Instruction::Not(not) => {
                        let wire_in = circuit_map
                            .get(&not.wire_in)
                            .expect("Missing value for not...");
                        
                        circuit_map.insert(not.wire_out.clone(), !*wire_in);
                    }
                    Instruction::Shift(shift) => {
                        let wire_in = circuit_map
                            .get(&shift.wire_in)
                            .expect("Missing value for shift...");
                        
                        if shift.operation == "LSHIFT" {
                            let result = *wire_in << shift.num;
                            circuit_map.insert(shift.wire_out.clone(), result);
                        } else {
                            let result = *wire_in >> shift.num;
                            circuit_map.insert(shift.wire_out.clone(), result);
                        }
                    }
                    Instruction::Logic(logic) => {
                        let wire_a = circuit_map
                            .get(&logic.wire_a)
                            .expect("Missing value for logic...");
                        
                        let wire_b = circuit_map
                            .get(&logic.wire_b)
                            .expect("Missing value for logic...");


                        if logic.operation == "AND" {
                            let result = *wire_a & *wire_b;
                            circuit_map.insert(logic.wire_out.clone(), result);
                        } else {
                            let result = *wire_a | *wire_b;
                            circuit_map.insert(logic.wire_out.clone(), result);
                        }
                    }
                    _ => {}
                }

                known_deps.push(wire.clone());
                cleared_instructions_indices.push(i);
            }
        }

        // remove finalized instructions, so that further iteration will be quicker
        for index in (0..cleared_instructions_indices.len()).rev() {
            instructions.remove(cleared_instructions_indices[index]);
        }

        // dbg!(&circuit_map);
    }

    println!("{:?}", &circuit_map);
    let a = String::from("a");
    let circuit_value = circuit_map.get(&a).expect("There is no value for a");
    println!("Final value for a: {}", circuit_value);

}
