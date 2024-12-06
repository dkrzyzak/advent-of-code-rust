use std::collections::{HashMap, VecDeque};

use crate::{common::{grid::Grid, point::Point}, parse_input};

mod extract;
use extract::*;

pub fn task() {
    let lines = parse_input!();
    let (grid, start) = extract_grid(&lines);

    let mut shortest_distances: HashMap<Point, usize> = HashMap::new();

    bfs_tiles(start.clone(), &grid, &mut shortest_distances, 64);

    let equal_tiles = shortest_distances
        .iter()
        .filter_map(|(_, distance)| {
            if distance % 2 == 0 {
                Some(*distance)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!("Distances len: {}", equal_tiles.len());
}

pub fn bfs_tiles(
    start: Point,
    grid: &Grid<bool>,
    distances: &mut HashMap<Point, usize>,
    how_many: usize,
) {
    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
    queue.push_back((start, 0));

    while !queue.is_empty() {
        let (point, steps) = queue.pop_front().unwrap();

        if !grid.contains_point(&point) {
            continue;
        }

        if grid.get_point(&point).unwrap() == false {
            continue;
        }

        if steps > how_many {
            continue;
        }

        if let Some(x) = distances.get(&point) {
            // we've already been there in earlier iteration - there is more efficient way
            // println!("this point is in the distances map");
            continue;
        } else {
            distances.insert(point, steps);
        }

        queue.push_back((point.east(), steps + 1));
        queue.push_back((point.west(), steps + 1));
        queue.push_back((point.north(), steps + 1));
        queue.push_back((point.south(), steps + 1));
    }
}
