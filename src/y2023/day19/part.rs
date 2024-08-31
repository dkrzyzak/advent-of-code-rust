use lazy_static::lazy_static;
use regex::Regex;
use crate::parse_captures;

#[derive(Debug)]
pub struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

lazy_static! {
    static ref PART_REG: Regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
}

impl Part {
    pub fn new(x: usize, m: usize, a: usize, s: usize) -> Part {
        Part { x, m, a, s }
    }

    pub fn parse(from: &String) -> Part {
        let (x, m, a, s) = parse_captures!(PART_REG, from, 1 usize, 2 usize, 3 usize, 4 usize);
        Part { x, m, a, s }
    }

    pub fn get_key(&self, key: &str) -> usize {
        match key {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => unreachable!("Wrong key! {key}")
        }
    }

    pub fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}
