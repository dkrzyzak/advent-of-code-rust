use crate::{common::point::Point, parse_input};

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

    let mut total_inside_area = 0u32;
    for row in 0..board_rows {
        let mut inside = false;
        let mut previous_connector = 'x';

        for col in 0..board_cols {
            let point = grid[row][col];

            if point == '.' && inside {
                // println!("Adding . point to area");
                total_inside_area += 1;
                continue;
            }

            let is_in_path = path.contains(&Point {
                x: col as i32,
                y: row as i32,
            });
            
            if !is_in_path && inside {
                // println!("Adding this point to area");
                total_inside_area += 1;
                continue;
            }

            if is_in_path {
                // !! HACK because in my data S is between | and | 
                if point == '|' || point == 'S' {
                    inside = !inside;
                }

                if point == '-' {
                    continue;
                }

                if point == 'L' || point == 'J' {
                    inside = !inside;
                    previous_connector = point;
                }

                if point == '7' && previous_connector == 'F' {
                    inside = !inside;
                    previous_connector = point;
                }

                if point == 'J' && previous_connector == 'L' {
                    inside = !inside;
                    previous_connector = point;
                }
            }

            
        }
    }

    println!("Total area: {}", total_inside_area);
}
