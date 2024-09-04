use crate::{common::algos::lcm_vec, parse_input};

mod circuit;
mod extract;
mod modules;
use circuit::*;
use extract::*;
use modules::*;

pub fn task() {
    let lines = parse_input!();
    let (mut circuit, mut final_circle) = extract_input(&lines);
    println!("Final circle: {:?}", final_circle);

    let mut cycles: Vec<u64> = Vec::new();
    let mut iterations_count = 0;

    loop {
        iterations_count += 1;
        push_button(&mut circuit, &mut cycles, &mut final_circle, iterations_count);

        if final_circle.is_empty() {
            break;
        }
    }

    println!("LCM of layers: {}", lcm_vec(&cycles));
}
