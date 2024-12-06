use std::collections::HashSet;

use crate::{
    common::{direction::Direction, grid::Grid, point::Point},
    parse_input,
};

pub fn task() {
    let lines = parse_input!();
    let grid = Grid::from_vec(&lines);

    let visited_points = part1(&grid);
    println!("Visited points: {}", visited_points);

    let total_loops = part2(&grid);
    println!("Total loops: {}", total_loops);
}

pub fn part1(grid: &Grid) -> usize {
    traverse(grid).unwrap()
}

pub fn part2(grid: &Grid) -> usize {
    let mut total_loops = 0;

    // there must be a better way than brute force...
    // ...but I choose the brute force
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if grid.get(row, col) != '.' {
                continue;
            }

            let mut new_grid = grid.clone();
            new_grid.overwrite(row, col, '#');
            let result = traverse(&new_grid);
            if result.is_none() {
                total_loops += 1;
            }
        }
    }

    total_loops
}

// returns number of all visited points when not ended up in a loop
// returns None if ended up in a loop
pub fn traverse(grid: &Grid) -> Option<usize> {
    let mut all_points: HashSet<Point> = HashSet::new();
    let mut blocked_points: HashSet<(Point, Direction)> = HashSet::new();
    let mut current_point = grid.find('^').unwrap();
    let mut direction = Direction::North;

    loop {
        all_points.insert(current_point);

        let next_point = current_point.next(direction);
        if let Some(next_symbol) = grid.get_point(&next_point) {
            if next_symbol != '#' {
                current_point = next_point;
            } else {
                let inserting_block_result = blocked_points.insert((next_point, direction));
                if !inserting_block_result {
                    // we've already been blocked there before going in the same direction
                    // we're in a loop
                    return None;
                }

                direction = direction.turn_right();
            }
        } else {
            // we went out of grid
            break;
        }
    }

    Some(all_points.len())
}
