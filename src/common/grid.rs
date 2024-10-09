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

    pub fn get(&self, row: usize, col: usize) -> char {
        self.data[row][col]
    }

    pub fn iget(&self, row: isize, col: isize) -> char {
        self.data[row as usize][col as usize]
    }
}
