use std::collections::{HashMap, HashSet};

use crate::{
    common::{direction::Direction, grid::Grid, point::Point},
    parse_input,
};

// type TrailMap = HashMap<Point, HashSet<Point>>; - PART 1
type TrailMap = HashMap<Point, HashSet<Vec<Point>>>;

pub fn task() {
    let lines = parse_input!();
    let map = Grid::u8_from_vec(&lines);
    let starting_points = map.find_all(0);
    let mut trail_map: TrailMap = HashMap::new();

    for start in starting_points.iter() {
        let current_trail = vec![start.clone()];

        find_trail(&map, start, 0, current_trail, &mut trail_map);
    }

    let score = calculate_score(&trail_map);
    println!("score: {:?}", score);
}

pub fn find_trail(
    grid: &Grid<u8>,
    current_point: &Point,
    current_height: u8,
    current_trail: Vec<Point>,
    trail_map: &mut TrailMap,
) {
    for dir in &[
        Direction::East,
        Direction::West,
        Direction::South,
        Direction::North,
    ] {
        let next_point = current_point.next(dir);
        if let Some(next_height) = grid.get_point(&next_point) {
            if next_height == current_height + 1 {
                let mut new_trail = current_trail.clone();
                new_trail.push(next_point);

                if next_height == 9 {
                    let first = current_trail.first().unwrap();
                    trail_map
                        .entry(*first)
                        .or_insert_with(HashSet::new)
                        // .insert(next_point) - PART 1
                        .insert(new_trail);

                    continue;
                }

                find_trail(grid, &next_point, next_height, new_trail, trail_map);
            }
        }
    }
}

pub fn calculate_score(trail_map: &TrailMap) -> usize {
    println!("{:?}", trail_map);
    trail_map.values().map(|points| points.len()).sum::<usize>()
}
