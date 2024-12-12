use std::collections::{HashSet, VecDeque};

use crate::common::{direction::DIRECTIONS, grid::Grid, point::Point};

pub type Region = HashSet<Point>;
pub type Regions = Vec<Region>;

pub fn extract_regions(grid_flowers: &Grid<char>, grid_visited: &mut Grid<bool>) -> Regions {
    let mut regions: Regions = Vec::new();
    let mut next_region_candidates: VecDeque<Point> = VecDeque::new();
    next_region_candidates.push_back(Point(0, 0));

    while let Some(point) = next_region_candidates.pop_front() {
        if let Some(visited) = grid_visited.get_point(&point) {
            if visited {
                continue;
            }
        }

        let current_flower = grid_flowers.get_point(&point).unwrap();

        let mut region_queue: VecDeque<Point> = VecDeque::new();
        region_queue.push_back(point);

        let mut region: Region = HashSet::new();

        while let Some(region_point) = region_queue.pop_front() {
            if !region.insert(region_point) {
                continue;
            }

            grid_visited.overwrite_point(&region_point, true);

            for dir in &DIRECTIONS {
                let new_point = region_point.next(dir);

                if let Some(visited) = grid_visited.get_point(&new_point) {
                    if visited {
                        continue;
                    }

                    let flower = grid_flowers.get_point(&new_point).unwrap();

                    if flower == current_flower {
                        region_queue.push_back(new_point);
                    } else {
                        next_region_candidates.push_back(new_point);
                    }
                }
            }
        }

        regions.push(region);
    }

    regions
}

pub fn calculate_fence_price(regions: &Regions) -> usize {
    regions
        .iter()
        .map(|region| region.len() * calculate_perimeter(region))
        .sum::<usize>()
}

pub fn calculate_perimeter(region: &Region) -> usize {
    let mut total = 0;
    for point in region.iter() {
        for dir in &DIRECTIONS {
            if !region.contains(&point.next(dir)) {
                total += 1;
            }
        }
    }

    total
}

pub fn calculate_sides(region: &Region) -> usize {
    let corner_count = 0;

    for &point in region {
        for dir in &DIRECTIONS {
            let adjacent = point.next(dir);
            if !region.contains(&adjacent) {}
        }
    }

    corner_count
}
