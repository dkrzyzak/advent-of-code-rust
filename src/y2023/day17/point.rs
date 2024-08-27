use std::borrow::Borrow;
use crate::common::direction::Direction;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub fn turn_left(&self, direction: &Direction) -> (Point, Direction) {
        match *direction {
            Direction::West => (Point(self.0 + 1, self.1), Direction::South),
            Direction::East => (Point(self.0 - 1, self.1), Direction::North),
            Direction::North => (Point(self.0, self.1 - 1), Direction::West),
            Direction::South => (Point(self.0, self.1 + 1), Direction::East),
        }
    }

    pub fn turn_right(&self, direction: &Direction) -> (Point, Direction) {
        match *direction {
            Direction::West => (Point(self.0 - 1, self.1), Direction::North),
            Direction::East => (Point(self.0 + 1, self.1), Direction::South),
            Direction::North => (Point(self.0, self.1 + 1), Direction::East),
            Direction::South => (Point(self.0, self.1 - 1), Direction::West),
        }
    }

    pub fn north(&self) -> Point {
        Point(self.0 - 1, self.1)
    }

    pub fn south(&self) -> Point {
        Point(self.0 + 1, self.1)
    }

    pub fn west(&self) -> Point {
        Point(self.0, self.1 - 1)
    }

    pub fn east(&self) -> Point {
        Point(self.0, self.1 + 1)
    }

    pub fn next(&self, direction: impl Borrow<Direction>) -> Point {
        match *direction.borrow() {
            Direction::West => self.west(),
            Direction::East => self.east(),
            Direction::North => self.north(),
            Direction::South => self.south(),
        }
    }

    pub fn is_valid(&self, rows: isize, cols: isize) -> bool {
        self.0 >= 0 && self.0 < rows && self.1 >= 0 && self.1 < cols
    }
}

impl Direction {
    pub fn index(&self) -> usize {
        match *self {
            Direction::North => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::East => 3,
        }
    }
}
