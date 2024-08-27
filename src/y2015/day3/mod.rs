use std::collections::HashMap;

use crate::{common::point::Point, parse_input};

pub fn task() {
    let chars_map = HashMap::from([
        ('^', Point(0, 1)),
        ('v', Point(0, -1)),
        ('<', Point(-1, 0)),
        ('>', Point(1, 0)),
    ]);

    let lines = parse_input!();
    let symbols = lines.first().unwrap();

    let mut current_santa = Point(0, 0);
    let mut current_robo = Point(0, 0);
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
