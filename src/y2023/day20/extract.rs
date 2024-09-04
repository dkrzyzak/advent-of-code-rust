use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use super::{Circuit, Module, ModuleType};

lazy_static! {
    static ref BROADCAST_REG: Regex = Regex::new(r"broadcaster -> (.*)").unwrap();
    static ref FLP_REG: Regex = Regex::new(r"%(\w+) -> (.*)").unwrap();
    static ref CON_REG: Regex = Regex::new(r"&(\w+) -> (.*)").unwrap();
}

pub fn extract_input(lines: &Vec<String>) -> Circuit  {
    let mut circuit = HashMap::<String, Module>::new();

    for line in lines.iter() {
        if let Some(captured) = BROADCAST_REG.captures(line) {
            let outputs = captured[1].parse::<String>().unwrap().split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
            let name = String::from("broadcast");

            circuit.insert(name.clone(), Module::new(ModuleType::Broadcast, name, outputs));
        }

        if let Some(captured) = FLP_REG.captures(line) {
            let name = captured[1].parse::<String>().unwrap();
            let outputs = captured[2].parse::<String>().unwrap().split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
            circuit.insert(name.clone(), Module::new(ModuleType::FlipFlop, name, outputs));
        }

        if let Some(captured) = CON_REG.captures(line) {
            let name = captured[1].parse::<String>().unwrap();
            let outputs = captured[2].parse::<String>().unwrap().split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
            circuit.insert(name.clone(), Module::new(ModuleType::Conjunction, name, outputs));
        }
    }

    for (name, module) in circuit.clone().iter_mut() {
        for output in module.outputs.iter() {
            if let Some(output_module) = circuit.get_mut(output) {
                output_module.inputs.insert(name.clone(), false);
            } else {
                circuit.insert(
                    output.clone(),
                    Module::new(ModuleType::FlipFlop, output.clone(), Vec::new()),
                );
            }
        }
    }

    circuit
}
