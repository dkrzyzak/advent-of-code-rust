#![allow(dead_code)]

use crate::parse_input;

fn prepare_data(lines: Vec<String>) -> Vec<(u32, u32)> {
   let parsed: Vec<Vec<u32>> = lines.iter().map(|line| {
      let (_, str1) = line.split_once(": ").unwrap_or_default();

      str1.split_whitespace()
        .map(|num| num.parse::<u32>().unwrap_or_default())
        .collect()
   }).collect();

   let mut races: Vec<(u32, u32)> = Vec::new();

   for i in 0..parsed[0].len() {
      races.push((parsed[0][i], parsed[1][i]));
   }

   races
}

pub fn task1() {
   let lines = parse_input!();
   let races = prepare_data(lines);
   
   let product = races.iter().map(|(time, distance_record)| {
      let mut record_count = 0;

      for speed in 0..=*time {
         let distance = speed * (time - speed);
         if distance > *distance_record {
            println!("distance = {distance} with speed = {speed}");
            record_count += 1;
         }
      }

      record_count
   }).fold(1, |sum, current| sum * current);

   dbg!(product);

}

pub fn task2() {
   // let lines = parse_input!();

   // SKIP THE VALIDATION, cause i can 

   // let time = 71530;
   // let distance_record: i64 = 940200;

   let time = 42686985;
   let distance_record: i64 = 284100511221341;

   // WOULD TAKE TOO MUCH WITH ITERATION
   // need to find first speed value where the record is beaten and the last one and perform subtraction
   // last - first + 1 (inclusive range)

   let low_threshold = {
      let mut speed = 0;
      while speed * (time - speed) < distance_record {
         speed += 1;
      }

      speed
   };

   let high_threshold = {
      let mut speed = time;
      while speed * (time - speed) < distance_record {
         speed -= 1;
      }

      speed
   };

      dbg!(low_threshold);
      dbg!(high_threshold);

      let value = high_threshold - low_threshold + 1;
      dbg!(value);
}