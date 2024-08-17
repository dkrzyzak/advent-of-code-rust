use std::time::Instant;

use helpers::{extract_input, find_location_for_seed};

use crate::parse_input;

mod mappings;
mod helpers;

pub fn task() {
   let mut lines = parse_input!();
   let (seeds, mappings) = extract_input(&mut lines);

   // PART 1
   // let seeds_at_locations = seeds.iter().map(|seed| find_location_for_seed(*seed, &mappings)).collect::<Vec<_>>();
   // let smallest_location = seeds_at_locations.iter().min().unwrap();

   // println!("Smallest location: {}", smallest_location);

   // PART 2
   let mut seeds_iter = seeds.iter();
   let mut all_seeds: Vec<u64> = Vec::new();
   let now = Instant::now();

   while let (Some(range_start), Some(range_len)) = (seeds_iter.next(), seeds_iter.next()) {
      for i in 0..*range_len {
         all_seeds.push(range_start + i);
      }
   }

   println!("Got all seeds...: {} in time {:?}", all_seeds.len(), now.elapsed());

   let seeds_at_locations = all_seeds.iter().map(|seed| find_location_for_seed(*seed, &mappings)).collect::<Vec<_>>();
   let smallest_location = seeds_at_locations.iter().min().unwrap();

   println!("Smallest location: {} calculated in {:?}", smallest_location, now.elapsed());

   // ACTUAL RESULT:
   // Got all seeds...: 2012333554 in time 16.3017077s
   // Smallest location: 24261545 calculated in 1867.1746023s
   // 30mins! that's crazy (but still faster than writing more optimal algorithm)
}
