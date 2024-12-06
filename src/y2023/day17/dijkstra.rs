use crate::common::{direction::Direction, grid::Grid, point::Point};
use std::{cmp::Reverse, collections::BinaryHeap};

type HeapData = (usize, Point, Direction, usize); // current cost, point, direction, how many moves in that direction

pub fn shortest_path(matrix: &Grid<usize>) -> Option<usize> {
    let start = Point(0, 0);
    let end = Point(matrix.irows - 1, matrix.icols - 1);

    // Distance matrix initialized with MAX (infinity)
    let mut dist = vec![vec![vec![vec![usize::MAX; 11]; 4]; matrix.cols]; matrix.rows];

    let mut queue: BinaryHeap<Reverse<HeapData>> = BinaryHeap::new();

    queue.push(Reverse((matrix.get(0, 1), Point(0, 1), Direction::East, 1)));
    queue.push(Reverse((matrix.get(1, 0), Point(1, 0), Direction::South, 1)));

    while let Some(Reverse(heap_data)) = queue.pop() {
        let (current_dist, current_point, last_dir, moves) = heap_data as HeapData;

        if current_point == end {
            return Some(current_dist);
        }

        let Point(row, col) = current_point;

        if current_dist > dist[row as usize][col as usize][last_dir.index()][moves - 1] {
            continue;
        }

        if moves < 10 {
            // Attempt to move in the same direction
            let next_point = current_point.next(&last_dir);
            if matrix.contains_point(&next_point) {
                let new_dist = current_dist + matrix.get_point(&next_point).unwrap();
                if new_dist < dist[next_point.0 as usize][next_point.1 as usize][last_dir.index()][moves] {
                    dist[next_point.0 as usize][next_point.1 as usize][last_dir.index()][moves] = new_dist;
                    queue.push(Reverse((new_dist, next_point, last_dir, moves + 1)));
                }
            }
        }

        if moves >= 4 {
            // Turn left and right and attempt those moves
            for (next_point, new_dir) in [
                current_point.turn_left(&last_dir),
                current_point.turn_right(&last_dir),
            ] {
                if matrix.contains_point(&next_point) {
                    let new_dist = current_dist + matrix.get_point(&next_point).unwrap();
                    if new_dist < dist[next_point.0 as usize][next_point.1 as usize][new_dir.index()][0] {
                        dist[next_point.0 as usize][next_point.1 as usize][new_dir.index()][0] = new_dist;
                        queue.push(Reverse((new_dist, next_point, new_dir, 1)));
                    }
                }
            }
        }
    }

    // If we exit the loop without reaching the end, return None
    None
}
