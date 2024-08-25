use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub row: isize,
    pub col: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Point {
    pub fn turn_left(&self, direction: &Direction) -> (Point, Direction) {
        match *direction {
            Direction::West => (Point { row: self.row + 1, col: self.col }, Direction::South),
            Direction::East => (Point { row: self.row - 1, col: self.col }, Direction::North),
            Direction::North => (Point { row: self.row, col: self.col - 1 }, Direction::West),
            Direction::South => (Point { row: self.row, col: self.col + 1 }, Direction::East),
        }
    }

    pub fn turn_right(&self, direction: &Direction) -> (Point, Direction) {
        match *direction {
            Direction::West => (Point { row: self.row - 1, col: self.col }, Direction::North),
            Direction::East => (Point { row: self.row + 1, col: self.col }, Direction::South),
            Direction::North => (Point { row: self.row, col: self.col + 1 }, Direction::East),
            Direction::South => (Point { row: self.row, col: self.col - 1 }, Direction::West),
        }
    }

    pub fn north(&self) -> Point {
        Point {
            row: self.row - 1,
            col: self.col,
        }
    }

    pub fn south(&self) -> Point {
        Point {
            row: self.row + 1,
            col: self.col,
        }
    }

    pub fn west(&self) -> Point {
        Point {
            row: self.row,
            col: self.col - 1,
        }
    }

    pub fn east(&self) -> Point {
        Point {
            row: self.row,
            col: self.col + 1,
        }
    }

    pub fn next(&self, direction: impl Borrow<Direction>) -> Point {
        match *direction.borrow() {
            Direction::West => self.west(),
            Direction::East => self.east(),
            Direction::North => self.north(),
            Direction::South => self.south(),
        }
    }
}