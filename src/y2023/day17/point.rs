use crate::common::{direction::Direction, point::Point};

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
}
