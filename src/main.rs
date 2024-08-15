#![allow(dead_code)]
#![allow(unused_variables)]

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
}

pub mod algos;
pub mod read;
pub mod write;

#[macro_export]
macro_rules! parse_input {
    () => {
        crate::read::read_input_file(std::path::Path::new(file!()).parent().unwrap())
    };
}
fn main() {
    y2015::day20::task();
}
