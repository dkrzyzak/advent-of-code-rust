#![allow(dead_code)]

use crate::read::read_input_file;

pub fn task1() {
   let lines = read_input_file("day9");

   let parsed: Vec<Vec<i32>> = lines.iter().map(|line| {
      line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect()
   }).collect();

   let mut total = 0;

   parsed.iter().for_each(|line| {
      let mut steps = 0;
      let mut steps_vec: Vec<Vec<i32>> = Vec::new();
      let mut current_line = line.clone();
      steps_vec.push(line.clone());

      loop {
         let diffs = current_line.windows(2).map(|window| window[1] - window[0]).collect::<Vec<i32>>();

         let all_zeros = diffs.iter().all(|diff| *diff == 0);

         steps_vec.push(diffs.clone());
         steps += 1;

         if all_zeros {
            break;
         }

         current_line = diffs;
      }

      // println!("steps_vec = {:?}", steps_vec);

      // we add a zero to the end of the last line
      steps_vec[steps].push(0);
      
      for i in (1..steps_vec.len()).rev() {
        let current_last =  steps_vec[i].last().unwrap();
        let prev_last = steps_vec[i - 1].last().unwrap();
        
        let new_item = current_last + prev_last;
        steps_vec[i - 1].push(new_item);

      }

      // println!("steps_vec = {:?}", steps_vec);
      let result = steps_vec[0].last().unwrap();
      // println!("result = {}", result);

      total += result;
   });

   println!("total = {}", total);
}

pub fn task2() {
   let lines = read_input_file("day9");

   let parsed: Vec<Vec<i32>> = lines.iter().map(|line| {
      line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect()
   }).collect();

   let mut total = 0;

   parsed.iter().for_each(|line| {
      let mut steps = 0;
      let mut steps_vec: Vec<Vec<i32>> = Vec::new();
      let mut current_line = line.clone();
      steps_vec.push(line.clone());

      loop {
         let diffs = current_line.windows(2).map(|window| window[1] - window[0]).collect::<Vec<i32>>();

         let all_zeros = diffs.iter().all(|diff| *diff == 0);

         steps_vec.push(diffs.clone());
         steps += 1;

         if all_zeros {
            break;
         }

         current_line = diffs;
      }

      // println!("steps_vec = {:?}", steps_vec);

      // we add a zero to the end of the last line
      steps_vec[steps].push(0);
      
      for i in (1..steps_vec.len()).rev() {
        let current_first =  steps_vec[i].first().unwrap();
        let prev_first = steps_vec[i - 1].first().unwrap();
        
        let new_item = prev_first - current_first;
        steps_vec[i - 1].insert(0, new_item);
      }

      // println!("steps_vec = {:?}", steps_vec);
      let result = steps_vec[0].first().unwrap();
      // println!("result = {}", result);

      total += result;
   });

   println!("total = {}", total);
}