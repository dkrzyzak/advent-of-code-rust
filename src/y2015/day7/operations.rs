use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct OInit {
    pub num: u16, // could also be a letter...
    pub wire: String,
}

#[derive(Debug, Clone)]
pub struct OAssign {
    pub wire_in: String,
    pub deps: HashSet<String>,
    pub wire_out: String,
}

#[derive(Debug, Clone)]
pub struct ONot {
    pub wire_in: String,
    pub deps: HashSet<String>,
    pub wire_out: String,
}

#[derive(Debug, Clone)]
pub struct OShift {
    pub wire_in: String,
    pub deps: HashSet<String>,
    pub operation: String, // LSHIFT or RSHIFT
    pub num: u16,
    pub wire_out: String,
}

#[derive(Debug, Clone)]
pub struct OLogic {
    pub wire_a: String,
    pub operation: String, // AND or OR
    pub wire_b: String,
    pub deps: HashSet<String>,
    pub wire_out: String,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Init(OInit),
    Assign(OAssign),
    Not(ONot),
    Shift(OShift),
    Logic(OLogic),
}

impl Instruction {
    pub fn deps(&mut self) -> Option<&mut HashSet<String>> {
        match self {
            Instruction::Init(_) => None,
            Instruction::Assign(assign) => Some(&mut assign.deps),
            Instruction::Not(not) => Some(&mut not.deps),
            Instruction::Shift(shift) => Some(&mut shift.deps),
            Instruction::Logic(logic) => Some(&mut logic.deps),
        }
    }

    pub fn wire_out(&self) -> &String {
        match self {
            Instruction::Init(init) => &init.wire,
            Instruction::Assign(assign) => &assign.wire_out,
            Instruction::Not(not) => &not.wire_out,
            Instruction::Shift(shift) => &shift.wire_out,
            Instruction::Logic(logic) => &logic.wire_out,
        }
    }
}
