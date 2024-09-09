use crate::common::point::Point;
use super::Grid;

pub fn extract_grid(lines: &Vec<String>) -> (Grid, Point) {
    let mut grid = Vec::new();
    let mut starting_point = Point::new(0, 0);

    for row in 0..lines.len() {
        let mut row_vec = Vec::new();
        let symbols = lines[row].chars().collect::<Vec<_>>();
    
        for col in 0..lines[0].len() {
            let symbol = symbols[col];

            match symbol {
                '#' => row_vec.push(false),
                '.' => row_vec.push(true),
                'S' => {
                    starting_point = Point::new(row as isize, col as isize);
                    row_vec.push(true);
                }
                _ => unreachable!("Invalid symbol")
            }
        }

        grid.push(row_vec);
    }

    (grid, starting_point)
}
