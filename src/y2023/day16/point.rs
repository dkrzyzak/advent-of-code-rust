use crate::common::{direction::Direction, point::Point};

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
