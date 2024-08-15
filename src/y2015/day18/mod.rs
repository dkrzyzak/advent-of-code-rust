use helpers::{count_neighbours, is_corner};

use crate::parse_input;

mod helpers;

pub fn task() {
    let lines = parse_input!();

    
    let mut grid = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let size = 100;
    let mut remaining_steps = 100;

    while remaining_steps > 0 {
        let mut new_grid = vec![vec!['.'; size]; size];

        for x in 0..size {
            for y in 0..size {
                // task 2:
                if is_corner(x, y, size) {
                    new_grid[x][y] = '#'; 
                    continue;
                }

                let light = grid[x][y];
                let is_on = if light == '#' { true } else { false };
                let (n_on, n_off) = count_neighbours(x, y, &grid, &size);

                if is_on {
                    new_grid[x][y] = if n_on == 2 || n_on == 3 { '#' } else { '.' };
                } else {
                    new_grid[x][y] = if n_on == 3 { '#' } else { '.' };
                }
            }
        }

        grid = new_grid;
        remaining_steps -= 1;
    }

    let how_many_on = grid
        .iter()
        .map(|row| row.iter().map(|ch| if *ch == '#' { 1 } else { 0 }).sum::<u32>())
        .sum::<u32>();
    
    println!("After all iterations {how_many_on} lights are on");
}

