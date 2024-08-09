use std::collections::HashMap;

use crate::read::read_input_file;

#[derive(Debug)]
struct Pair {
   a: u32,
   b: u32,
   distance: u32,
}

impl Pair {
   pub fn new(a: u32, b: u32, distance: u32) -> Self {
       Pair { a, b, distance }
   }

   pub fn contains(&self, a: u32, b: u32) -> bool {
       (self.a == a && self.b == b) || (self.a == b && self.b == a)
   }
}

pub fn task1() {
    let mut lines: Vec<Vec<char>> = read_input_file("y2023", "day11")
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    expand_universe(&mut lines);

    println!("Expanded. Creating map of galaxies");

   // gather galaxies into a map of u32 (index) -> (row, col)
   let mut galaxies: HashMap<u32, (usize, usize)> = HashMap::new();

   for (row_index, row) in lines.iter().enumerate() {
      for (col_index, col) in row.iter().enumerate() {
         if *col == '#' {
            galaxies.insert(galaxies.len() as u32, (row_index, col_index));
         }
      }
   }

   println!("Galaxies: {:?}", galaxies);


   let mut pairs: Vec<Pair> = Vec::new();

   for (a, a_coords) in galaxies.iter() {
      for (b, b_coords) in galaxies.iter() {
         if a != b && !pairs.iter().any(|pair| pair.contains(*a, *b)) {

            let distance = (a_coords.0 as i32 - b_coords.0 as i32).abs() + (a_coords.1 as i32 - b_coords.1 as i32).abs();
            println!("Distance between {:?} and {:?} is {:?}", a, b, distance);
            pairs.push(Pair::new(*a, *b, distance as u32));
         }
      }
   }

   println!("Pairs len: {:?}", pairs.len());

   let total_distance = pairs.iter().fold(0, |total, current_pair| total + current_pair.distance);
   println!("Total distance: {:?}", total_distance);

}

fn expand_universe(universe: &mut Vec<Vec<char>>) {
   let mut empty_rows = Vec::new();
   let mut empty_cols = Vec::new();

   for row in 0..universe.len() {
      if universe[row].iter().all(|&c| c == '.') {
         empty_rows.push(row);
      }
   }

   for col in 0..universe[0].len() {
      if universe.iter().all(|row| row[col] == '.') {
         empty_cols.push(col);
      }
   }

   println!("Empty rows: {:?}", empty_rows);
   println!("Empty cols: {:?}", empty_cols);

   // EXPAND COLS
   for (index, col) in empty_cols.iter().enumerate() {
      for row in universe.iter_mut() {
         // we add index to offset the previously inserted cols
         row.insert(*col + index, '.');
      }
   }

   for (index, row) in empty_rows.iter().enumerate() {
      // we add index to offset the previously inserted rows
      universe.insert(*row + index, vec!['.'; universe[0].len()]);
   }

   // write_to_file(universe.clone(), "src/day11/expanded").expect("Unable to write");
}
