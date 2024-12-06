use std::collections::HashSet;

use crate::{
    common::{direction::Direction, grid::Grid, point::Point},
    parse_input,
};

pub fn task() {
    let lines = parse_input!();
    let grid = Grid::from_vec(&lines);

    let mut all_points: HashSet<Point> = HashSet::new();
    let mut current_point = grid.find('^').unwrap(); // we know this symbol is in the data
    let mut direction = Direction::North;

    loop {
        all_points.insert(current_point);

        let next_point = current_point.next(direction);
        if let Some(next_symbol) = grid.get_point(&next_point) {
            if next_symbol != '#' {
                current_point = next_point;
            } else {
                direction = direction.turn_right();
            }
        } else {
            // we went out of grid
            break;
        }
    }

    println!("Visited points: {}", all_points.len());
}
