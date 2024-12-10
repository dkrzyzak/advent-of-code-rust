use super::point::Point;

#[derive(Debug, Clone)]
pub struct Grid<T = char> {
    pub data: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
    pub irows: isize,
    pub icols: isize,
}

impl<T> Grid<T>
where
    T: Copy + PartialEq,
{
    pub fn initialize(data: Vec<Vec<T>>) -> Grid<T> {
        let rows = data.len();
        let cols = data[0].len();

        Grid {
            data,
            rows,
            cols,
            irows: rows as isize,
            icols: cols as isize,
        }
    }

    pub fn contains(&self, row: usize, col: usize) -> bool {
        return row < self.rows && col < self.cols;
    }

    pub fn icontains(&self, row: isize, col: isize) -> bool {
        return row >= 0 && (row as usize) < self.rows && col >= 0 && (col as usize) < self.cols;
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        return point.0 >= 0
            && (point.0 as usize) < self.rows
            && point.1 >= 0
            && (point.1 as usize) < self.cols;
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row][col]
    }

    pub fn iget(&self, row: isize, col: isize) -> Option<T> {
        if !self.icontains(row, col) {
            return None;
        }

        Some(self.data[row as usize][col as usize])
    }

    pub fn get_point(&self, point: &Point) -> Option<T> {
        if !self.contains_point(point) {
            return None;
        }

        return Some(self.data[point.0 as usize][point.1 as usize]);
    }

    pub fn find(&self, search: T) -> Option<Point> {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.data[row][col] == search {
                    return Some(Point(row as isize, col as isize));
                }
            }
        }

        return None;
    }

    pub fn find_all(&self, search: T) -> Vec<Point> {
        let mut found = Vec::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.data[row][col] == search {
                    found.push(Point(row as isize, col as isize));
                }
            }
        }

        found
    }

    pub fn overwrite(&mut self, row: usize, col: usize, value: T) {
        self.data[row][col] = value;
    }
}

// implement initializing:

impl Grid<char> {
    pub fn from_vec(input: &Vec<String>) -> Grid<char> {
        let data = input
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Grid::initialize(data)
    }
}

impl Grid<usize> {
    pub fn usize_from_vec(input: &Vec<String>) -> Grid<usize> {
        let data = input
            .iter()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Grid::initialize(data)
    }
}

// todo: make generic Numeric trait and 
impl Grid<u8> {
    pub fn u8_from_vec(input: &Vec<String>) -> Grid<u8> {
        let data = input
            .iter()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Grid::initialize(data)
    }
}

impl Grid<bool> {
    pub fn map_bool_from_vec<Mapper>(input: &Vec<String>, mapper: Mapper) -> Grid<bool>
    where
        Mapper: Fn(char) -> bool,
    {
        let data = input
            .iter()
            .map(|line| line.chars().map(|ch| mapper(ch)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Grid::initialize(data)
    }
}
