use lazy_static::lazy_static;
use regex::Regex;

use crate::parse_captures;

lazy_static! {
    static ref REG: Regex = Regex::new(r"(\w) (\w)").unwrap();
}

#[derive(Debug, PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn value(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn fight(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
            
            (Shape::Rock, Shape::Scissors) => Outcome::Win,
            (Shape::Scissors, Shape::Rock) => Outcome::Lose,

            (Shape::Paper, Shape::Rock) => Outcome::Win,
            (Shape::Rock, Shape::Paper) => Outcome::Lose,

            (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (Shape::Paper, Shape::Scissors) => Outcome::Lose,
        }
    }
}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn value(&self) -> u8 {
        match &self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

pub fn extract_shapes(lines: &Vec<String>) -> Vec<(Shape, Shape)> {
    lines.iter().map(|line| {
        let (elf_letter, my_letter) = parse_captures!(REG, line, 1 char, 2 char);

        let elf_shape = match elf_letter {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => unreachable!(),
        };

        let my_shape = match my_letter {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => unreachable!(),
        };

        (elf_shape, my_shape)
    }).collect::<Vec<_>>()
}


