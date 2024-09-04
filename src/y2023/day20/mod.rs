use crate::parse_input;

mod circuit;
mod extract;
mod modules;
use circuit::*;
use extract::*;
use modules::*;

pub fn task() {
    let lines = parse_input!();
    let mut circuit: Circuit = extract_input(&lines);

    let mut high_signals = 0;
    let mut low_signals = 0;

    for _ in 0..1000 {
        push_button(&mut circuit, &mut high_signals, &mut low_signals);
    }

    println!("H: {}, L: {}", high_signals, low_signals);
    println!("Multiplication: {}", high_signals * low_signals);
}
