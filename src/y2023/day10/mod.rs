use std::collections::HashMap;

use crate::parse_input;

// const BOARD_SIZE: usize = 140;
const BOARD_SIZE: (usize, usize) = (20, 10);

const TASK2 : bool = true;

fn find_start(parsed: &Vec<Vec<char>>) -> (usize, usize) {
   let mut s_index: (usize, usize) = (0, 0); 

   for (y_index, inner_vec) in parsed.iter().enumerate() {
      for (x_index, &item) in inner_vec.iter().enumerate() {
         if item == 'S' {
            s_index = (x_index, y_index);
            break;
         }
      }
   }

   s_index
}

#[derive(Debug)]
struct Neighbour {
   vector: (i32, i32),
   allowed_symbols: Vec<char>,
}

pub fn task1() {
   let lines = parse_input!();
   let parsed: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

   let s_index = find_start(&parsed);

   let map = HashMap::from([
   ('|', vec![
   Neighbour { vector: (0, 1), allowed_symbols: vec!['|', 'L', 'J', 'S'] },
   Neighbour { vector: (0, -1), allowed_symbols: vec!['|', 'F', '7', 'S']},
   ]),

   ('-', vec![ 
   Neighbour { vector: (1, 0), allowed_symbols: vec!['-', 'J', '7', 'S'] },
   Neighbour { vector: (-1, 0), allowed_symbols: vec!['-', 'L', 'F', 'S']},
   ]),

   ('L', vec![
   Neighbour { vector: (0, -1), allowed_symbols: vec!['|', 'F', '7'] },
   Neighbour { vector: (1, 0), allowed_symbols: vec!['-', 'J', '7', 'S']},
   ]),

   ('J', vec![
   Neighbour { vector: (0, -1), allowed_symbols: vec!['|', 'F', '7', 'S'] },
   Neighbour { vector: (-1, 0), allowed_symbols: vec!['-', 'F', 'L', 'S']},
   ]),

   ('F', vec![
   Neighbour { vector: (1, 0), allowed_symbols: vec!['-', '7', 'J', 'S'] },
   Neighbour { vector: (0, 1), allowed_symbols: vec!['|', 'L', 'J', 'S']},
   ]),

   ('7', vec![
   Neighbour { vector: (-1, 0), allowed_symbols: vec!['-', 'F', 'L', 'S'] },
   Neighbour { vector: (0, 1), allowed_symbols: vec!['|', 'L', 'J', 'S']},
   ]),

   ('S', vec![
   Neighbour { vector: (1, 0), allowed_symbols: vec!['-', 'J', '7'] },
   Neighbour { vector: (-1, 0), allowed_symbols: vec!['-', 'F', 'L']},
   Neighbour { vector: (0, 1), allowed_symbols: vec!['|', 'L', 'J']},
   Neighbour { vector: (0, -1), allowed_symbols: vec!['|', 'F', '7']},
   ]),
   
   ]);

   print!("s index = {:?}", s_index);

   let mut distance = 0;
   let mut visited: Vec<(usize, usize)> = Vec::new();
   let mut current_idx = (s_index.0, s_index.1);

   loop {
      let current_symbol = parsed[current_idx.1][current_idx.0];

      let neighbours = map.get(&current_symbol).unwrap();

      'find: for neighbour in neighbours {
         let new_x = current_idx.0 as i32 + neighbour.vector.0;
         let new_y = current_idx.1 as i32 + neighbour.vector.1;

         if new_x < 0 || new_y < 0 || new_x >= BOARD_SIZE.0 as i32 || new_y >= BOARD_SIZE.1 as i32 {
            continue;
         }

         let new_x = new_x as usize;
         let new_y = new_y as usize;


         if visited.contains(&(new_x, new_y)) {
            continue;
         }

         let new_symbol = parsed[new_y][new_x];

         for allowed_symbol in &neighbour.allowed_symbols {
            if new_symbol == *allowed_symbol {
               current_idx = (new_x, new_y);
               visited.push(current_idx);
               distance += 1;

               break 'find;
            }
         }
         
      }

      if parsed[current_idx.1][current_idx.0] == 'S' {
         break;
      }
      
   }

   println!("distance = {}", distance);

   if !TASK2 {
      return;
   }

   let mut area = 0;
   let mut area_tiles = Vec::new();

   for (y_index, line) in parsed.iter().enumerate() {
      let mut inside = false;

      for (x_index, &item) in line.iter().enumerate() {
         if !visited.contains(&(x_index, y_index)) {
            if inside {
               area += 1;
               area_tiles.push((x_index, y_index));
            } else {
               continue;
            }
         } else {
            if item != '-' {
               inside = !inside;
            }
            continue;
         }

      }
         
   }

   println!("area = {}", area);
   println!("area tiles = {:?}", area_tiles);

}


