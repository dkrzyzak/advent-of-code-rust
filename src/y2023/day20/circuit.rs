use super::Module;
use std::collections::{HashMap, HashSet, VecDeque};

pub type Circuit = HashMap<String, Module>;
pub type FinalCircle = HashSet<String>;

pub fn push_button(
    circuit: &mut Circuit,
    cycles: &mut Vec<u64>,
    final_circle: &mut FinalCircle,
    iter_count: u64,
) {
    let mut queue: VecDeque<(String, bool)> = VecDeque::new();

    // sending low signal from button to broadcast
    queue.push_back((String::from("broadcast"), false));

    while !queue.is_empty() {
        let (module_name, state) = queue.pop_front().unwrap();

        let module = circuit.get_mut(&module_name).unwrap();
        if let Some(new_state) = module.new_state(state) {
            let outputs_to_add = module.outputs.clone();

            // BULK OF PART 2 LOGIC
            if final_circle.contains(&module_name) && new_state {
                cycles.push(iter_count);
                final_circle.remove(&module_name);
            }

            for output in outputs_to_add {
                if let Some(output_module) = circuit.get_mut(&output) {
                    output_module.inputs.insert(module_name.clone(), new_state); // insert actually just overwrites existing value

                    // println!("{} -{}-> {}", module_name, new_state, output);

                    queue.push_back((output, new_state));
                }
            }
        }
    }
}
