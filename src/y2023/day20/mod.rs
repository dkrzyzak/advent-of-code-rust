use std::collections::{HashMap, HashSet, VecDeque};

use crate::parse_input;

mod extract;
mod modules;
use extract::*;
use modules::*;

pub fn task() {
    let lines = parse_input!();
    let mut circuit: HashMap<String, Module> = extract_input(&lines);

    let mut signals: HashSet<(String, String)> = HashSet::new();
    let mut queue: VecDeque<(String, bool)> = VecDeque::new();
    let mut high_signals = 0;
    let mut low_signals = 1;
    // circuit.insert("output".to_string(), Module::new(ModuleType::FlipFlop, "output".to_string(), Vec::new()));
    queue.push_back((String::from("broadcast"), false));

    while !queue.is_empty() {
        println!("Queue: {:?}", queue);
        let (module_name, state) = queue.pop_front().unwrap();

        // println!("Popped: {}, {}", module_name, state);

        let module = circuit.get_mut(&module_name).unwrap();
        let new_state = module.new_state(state);

        let outputs_to_add = module.outputs.clone();

        for output in outputs_to_add {
            if let Some(output_module) = circuit.get_mut(&output) {
                output_module.inputs.insert(module_name.clone(), new_state);

                if signals.insert((module_name.clone(), output.clone())) {
                    println!("{} -{}-> {}", module_name, new_state, output);

                    queue.push_back((output, new_state));
                }
            }
        }
    }
}
