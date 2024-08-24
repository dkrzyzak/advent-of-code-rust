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

pub fn splitter(
    splitter_type: char,
    point: &Point,
    direction: &Direction,
) -> Option<((Point, Direction), (Point, Direction))> {
    if splitter_type == '-' {
        match direction {
            Direction::East | Direction::West => return None,
            Direction::North | Direction::South => {
                let next_point_east = point.east();
                let next_point_west = point.west();

                return Some((
                    (next_point_east, Direction::East),
                    (next_point_west, Direction::West),
                ));
            }
        }
    }

    if splitter_type == '|' {
        match direction {
            Direction::North | Direction::South => return None,
            Direction::West | Direction::East => {
                let next_point_north = point.north();
                let next_point_south = point.south();

                return Some((
                    (next_point_north, Direction::North),
                    (next_point_south, Direction::South),
                ));
            }
        }
    }

    panic!("Unsupported splitter: {}", splitter_type);
}

pub fn mirror(mirror_type: char, point: &Point, direction: &Direction) -> (Point, Direction) {
    if mirror_type == '\\' {
        return match direction {
            Direction::East => (point.south(), Direction::South),
            Direction::West => (point.north(), Direction::North),
            Direction::North => (point.west(), Direction::West),
            Direction::South => (point.east(), Direction::East),
        };
    }

    if mirror_type == '/' {
        return match direction {
            Direction::East => (point.north(), Direction::North),
            Direction::West => (point.south(), Direction::South),
            Direction::North => (point.east(), Direction::East),
            Direction::South => (point.west(), Direction::West),
        };
    }

    panic!("Unsupported mirror type: {}", mirror_type);
}
