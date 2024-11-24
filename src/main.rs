#![allow(dead_code)]
#![allow(unused_variables)]

use common::new_day;
use std::env;

mod y2015 {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}

mod y2022 {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
}

mod y2023 {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
}

mod common {
    pub mod new_day;
    pub mod write;
    pub mod read;
    pub mod point;
    pub mod grid;
    pub mod direction;
    pub mod print_by_line;
    pub mod algos;
    pub mod regex_macro;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(_) = new_day::resolve_args(&args) {
        return;
    }

    y2022::day7::task();
}
