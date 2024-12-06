use super::point::Point;

#[derive(Debug, Clone)]
pub struct Grid {
    pub data: Vec<Vec<char>>,
    pub rows: usize,
    pub cols: usize,
    pub irows: isize,
    pub icols: isize,
}

// impl<T> Grid<T> where T: Clone

impl Grid {
    pub fn from_vec(v: &Vec<String>) -> Grid {
        let grid = v
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let rows = grid.len();
        let cols = grid[0].len();

        Grid {
            data: grid,
            rows,
            cols,
            irows: rows as isize,
            icols: cols as isize,
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        return point.0 >= 0
            && (point.0 as usize) < self.rows
            && point.1 >= 0
            && (point.1 as usize) < self.cols;
    }

    pub fn get(&self, row: usize, col: usize) -> char {
        self.data[row][col]
    }

    // TODO: use self.contains
    pub fn iget(&self, row: isize, col: isize) -> Option<char> {
        if row < 0 || row >= self.irows || col < 0 || col >= self.icols {
            return None;
        }

        Some(self.data[row as usize][col as usize])
    }

    pub fn get_point(&self, point: &Point) -> Option<char> {
        if !self.contains(point) {
            return None;
        }

        return Some(self.data[point.0 as usize][point.1 as usize]);
    }

    pub fn find(&self, search: char) -> Option<Point> {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.data[row][col] == search {
                    return Some(Point::new(row as isize, col as isize));
                }
            }
        }

        return None;
    }
}
