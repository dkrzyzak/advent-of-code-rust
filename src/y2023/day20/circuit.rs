use super::Module;
use std::collections::{HashMap, VecDeque};

pub type Circuit = HashMap<String, Module>;

pub fn push_button(circuit: &mut Circuit, high_signals: &mut usize, low_signals: &mut usize) {
    let mut queue: VecDeque<(String, bool)> = VecDeque::new();

    // sending low signal from button to broadcast
    *low_signals += 1;
    queue.push_back((String::from("broadcast"), false));

    while !queue.is_empty() {
        let (module_name, state) = queue.pop_front().unwrap();

        let module = circuit.get_mut(&module_name).unwrap();
        if let Some(new_state) = module.new_state(state) {
            let outputs_to_add = module.outputs.clone();

            for output in outputs_to_add {
                if let Some(output_module) = circuit.get_mut(&output) {
                    output_module.inputs.insert(module_name.clone(), new_state); // insert actually just overwrites existing value

                    // println!("{} -{}-> {}", module_name, new_state, output);

                    if new_state {
                        *high_signals += 1;
                    } else {
                        *low_signals += 1;
                    }

                    queue.push_back((output, new_state));
                }
            }
        }
    }
}
