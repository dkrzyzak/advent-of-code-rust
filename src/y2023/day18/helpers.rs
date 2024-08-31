use std::collections::{HashSet, VecDeque};

use crate::common::point::Point;

impl Point {
    pub fn min_x(points: &Vec<Point>) -> isize {
        points.iter().map(|p| p.0).min().unwrap()
    }

    pub fn min_y(points: &Vec<Point>) -> isize {
        points.iter().map(|p| p.1).min().unwrap()
    }

    pub fn max_x(points: &Vec<Point>) -> isize {
        points.iter().map(|p| p.0).max().unwrap()
    }

    pub fn max_y(points: &Vec<Point>) -> isize {
        points.iter().map(|p| p.1).max().unwrap()
    }

    pub fn bounding_box(points: &Vec<Point>) -> (isize, isize, isize, isize) {
        let min_x = Self::min_x(points);
        let max_x = Self::max_x(points);
        let min_y = Self::min_y(points);
        let max_y = Self::max_y(points);

        (min_x, min_y, max_x, max_y)
    }
}

pub fn shoelace_formula(points: &Vec<Point>) -> isize {
    let len = points.len();

    let (area, perimeter) =
        points
            .iter()
            .enumerate()
            .fold((0isize, 0isize), |(sum, perimeter), (i, p1)| {
                let l = (i + 1) % len;
                let p2 = points[l];

                let distance = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
                let new_perimeter = perimeter + distance;
                let new_area = sum + (p1.1 * p2.0) - (p1.0 * p2.1);

                (new_area, new_perimeter)
            });

    area.abs() + (perimeter / 2) + 1
}

pub fn flood_fill(perimeter: &Vec<Point>) -> Vec<Point> {
    let (min_x, min_y, max_x, max_y) = Point::bounding_box(perimeter);
    println!("Bounding box: ({}, {}) x ({}, {})", min_x, min_y, max_x, max_y);

    let mut result = perimeter.clone();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // start flood-fill from the top-left corner of the bounding box
    queue.push_back(Point(1, 1));

    while let Some(point) = queue.pop_front() {
        if point.0 < min_x || point.0 > max_x || point.1 < min_y || point.1 > max_y {
            continue;
        }

        if visited.contains(&point) || perimeter.contains(&point) {
            continue;
        }

        visited.insert(point);
        result.push(point);

        queue.push_back(point.north());
        queue.push_back(point.south());
        queue.push_back(point.west());
        queue.push_back(point.east());

        // queue.push_back(Point(point.0 + 1, point.1));
        // queue.push_back(Point(point.0 - 1, point.1));
        // queue.push_back(Point(point.0, point.1 + 1));
        // queue.push_back(Point(point.0, point.1 - 1));
    }

    result
}
// results so far:

// 11697
// 17101
// 26857
