#![allow(dead_code)]

use std::collections::HashMap;

use crate::common::algos::lcm_vec;
use crate::parse_input;

fn prepare_data() -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut lines = parse_input!();
    // EXTRACT L-R INSTRUCTIONS
    let instructions = lines.get(0).unwrap().chars().collect::<Vec<char>>();

    // REMOVE REDUNDANT LINES
    lines.remove(0);
    lines.remove(0);

    let mut network: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        let key = parts[0].to_string();
        let second_last = parts[1].len() - 1;
        let values: Vec<&str> = parts[1][1..second_last].split(", ").collect();
        let value = (values[0].to_string(), values[1].to_string());
        network.insert(key, value);
    }

    (instructions, network)
}

pub fn task1() {
    let (instructions, network) = prepare_data();

    let mut instructions_idx = 0;
    let mut steps = 0;
    let mut node = String::from("AAA");

    loop {
        if instructions_idx >= instructions.len() {
            instructions_idx = 0;
        }

        let instruction = instructions[instructions_idx];
        let (left, right) = network.get(&node).unwrap();

        // println!("instruction = {}, node = {}, left = {}, right = {}", instruction, node, left, right);

        if instruction == 'L' {
            node = left.clone()
        } else if instruction == 'R' {
            node = right.clone();
        }

        steps += 1;

        if node == "ZZZ" {
            break;
        }

        instructions_idx += 1;
    }

    println!("steps = {}", steps);
}


pub fn task2() {
   let (instructions, network) = prepare_data();
    let nodes: Vec<String> = network
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|key| key.clone())
        .collect();

   let mut steps_count: Vec<u64> = Vec::new();

   for node in nodes.iter() {
      let mut instructions_idx = 0;
      let mut steps = 0;
      let mut node = node.clone();

      loop {
         if instructions_idx >= instructions.len() {
               instructions_idx = 0;
         }

         let instruction = instructions[instructions_idx];
         let (left, right) = network.get(&node).unwrap();

         // println!("instruction = {}, node = {}, left = {}, right = {}", instruction, node, left, right);

         if instruction == 'L' {
               node = left.clone();
         } else if instruction == 'R' {
               node = right.clone();
         }

         steps += 1;

         if node.ends_with("Z") {
               break;
         }

         instructions_idx += 1;
      }

      steps_count.push(steps);
   }

   println!("steps_count = {:?}", steps_count);
   let lcm = lcm_vec(steps_count);
   println!("lcm = {:?}", lcm);
}