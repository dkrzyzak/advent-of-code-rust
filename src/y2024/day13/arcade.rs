use lazy_static::lazy_static;
use regex::Regex;

use crate::parse_captures;

lazy_static! {
    static ref BUTTON_A_REG: Regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    static ref BUTTON_B_REG: Regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    static ref PRIZE_REG: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
}

#[derive(Debug)]
pub struct Machine {
    pub ax: u64,
    pub ay: u64,
    pub bx: u64,
    pub by: u64,
    pub prize_x: u64,
    pub prize_y: u64,
}

#[derive(Debug, Copy, Clone)]
pub enum Button {
    A,
    B,
}

pub fn extract_machines(lines: &Vec<String>, offset: u64) -> Vec<Machine> {
    let mut iter = lines.iter();
    let mut machines = Vec::new();

    while let (Some(button_a_line), Some(button_b_line), Some(prize_line)) =
        (iter.next(), iter.next(), iter.next())
    {
        let (ax, ay) = parse_captures!(BUTTON_A_REG, button_a_line, 1 u64, 2 u64);
        let (bx, by) = parse_captures!(BUTTON_B_REG, button_b_line, 1 u64, 2 u64);
        let (prize_x, prize_y) = parse_captures!(PRIZE_REG, prize_line, 1 u64, 2 u64);

        machines.push(Machine {
            ax,
            ay,
            bx,
            by,
            prize_x: prize_x + offset,
            prize_y: prize_y + offset,
        });

        // skip the empty line
        iter.next();
    }

    machines
}
