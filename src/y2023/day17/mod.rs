use crate::{common::grid::Grid, parse_input};

mod point;
mod dijkstra;
use dijkstra::*;

pub fn task() {
    let lines = parse_input!();
    let map = Grid::usize_from_vec(&lines);

    match shortest_path(&map) {
        Some(cost) => println!("The cost of the cheapest path is: {}", cost),
        None => println!("No path found."),
    }
}
