use std::{collections::HashMap, ops::Add};

use crate::parse_input;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> Add<&'a Point> for Point {
    type Output = Point;

    fn add(self, other: &'a Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn task() {
    let chars_map = HashMap::from([
        ('^', Point { x: 0, y: 1 }),
        ('v', Point { x: 0, y: -1 }),
        ('<', Point { x: -1, y: 0 }),
        ('>', Point { x: 1, y: 0 }),
    ]);

    let lines = parse_input!();
    let symbols = lines.first().unwrap();

    let mut current_santa = Point { x: 0, y: 0 };
    let mut current_robo = Point { x: 0, y: 0 };
    let mut santas_move = true;

    let mut visited: HashMap<Point, u32> = HashMap::new();

    visited.insert(current_santa.clone(), 1);
    visited.insert(current_robo.clone(), 1);

    for direction in symbols.chars() {
        let move_vec = chars_map.get(&direction).unwrap();
        let current: &Point;

        if santas_move {
            current_santa = current_santa + move_vec;
            current = &current_santa;
        } else {
            current_robo = current_robo + move_vec;
            current = &current_robo;
        }

        *visited.entry(*current).or_insert(0) += 1;

      //   match visited.get(current) {
      //       None => {
      //           visited.insert(current.clone(), 1);
      //       }
      //       Some(already_delivered) => {
      //           *visited.get_mut(current).unwrap() += 1;
      //       }
      //   }

        santas_move = !santas_move;
    }

    dbg!(visited.keys().len());
}
