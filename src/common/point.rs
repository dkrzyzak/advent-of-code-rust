use super::direction::Direction;
use std::{
    borrow::Borrow,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub fn new(row: isize, col: isize) -> Self {
        Point(row, col)
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
}

// Point + Point
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

// Point + &Point
impl<'a> Add<&'a Point> for Point {
    type Output = Point;

    fn add(self, other: &'a Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

// Point - Point
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

// Point - &Point
impl<'a> Sub<&'a Point> for Point {
    type Output = Point;

    fn sub(self, other: &'a Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

// &Point - &Point
impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}
