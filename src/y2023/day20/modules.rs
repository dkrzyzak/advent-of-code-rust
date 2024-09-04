use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Module {
    pub module_type: ModuleType,
    pub name: String,
    pub state: bool,
    pub inputs: HashMap<String, bool>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

impl Module {
    pub fn new(module_type: ModuleType, name: String, outputs: Vec<String>) -> Self {
        Module {
            module_type,
            name,
            outputs,
            state: false,
            inputs: HashMap::new(),
        }
    }

    pub fn new_state(&mut self, signal: bool) -> bool {
        let new_state = match self.module_type {
            ModuleType::Broadcast => false,
            ModuleType::FlipFlop => {
                if signal { self.state } else { !self.state }
            }
            ModuleType::Conjunction => {
                let all_highs = self.inputs.values().all(|value| *value);

                if all_highs { false } else { true }
            }
        };

        self.state = new_state;

        new_state
    }
}
