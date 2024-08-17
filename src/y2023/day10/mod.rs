use crate::parse_input;

mod helpers;
use helpers::*;

pub fn task() {
    let lines = parse_input!();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let board_rows = grid.len();
    let board_cols = grid[0].len();
    let board_size = (board_cols, board_rows);

    let path = part_1(&grid, &board_size);

    // TASK 2

    //  for x in 0..board_rows {
    //      for y in 0..board_cols {
    //          let ch = grid[x][y];
    //          let is_in_path = path.contains(&Point {
    //              x: x as i32,
    //              y: y as i32,
    //          });
    //          println!(
    //             //  "index: x: {} y: {}, ch: {}, is in path: {}",
    //              x, y, ch, is_in_path
    //          );
    //      }
    //      print!("\n");
    //  }

    // let mut area = 0;
    // let mut area_tiles = Vec::new();

    // for (y_index, line) in parsed.iter().enumerate() {
    //    let mut inside = false;

    //    for (x_index, &item) in line.iter().enumerate() {
    //       if !visited.contains(&(x_index, y_index)) {
    //          if inside {
    //             area += 1;
    //             area_tiles.push((x_index, y_index));
    //          } else {
    //             continue;
    //          }
    //       } else {
    //          if item != '-' {
    //             inside = !inside;
    //          }
    //          continue;
    //       }

    //    }

    // }

    // println!("area = {}", area);
    // println!("area tiles = {:?}", area_tiles);
}
