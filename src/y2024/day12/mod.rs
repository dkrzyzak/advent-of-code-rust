use crate::{common::grid::Grid, parse_input};

mod garden;
use garden::*;

pub fn task() {
    let lines = parse_input!();
    let grid_flowers = Grid::from_vec(&lines);
    let mut grid_visited = Grid::map_bool_from_vec(&lines, |_| false);

    let regions = extract_regions(&grid_flowers, &mut grid_visited);
    let fence_price = calculate_fence_price(&regions);
    println!("Fence price: {}", fence_price);
}
