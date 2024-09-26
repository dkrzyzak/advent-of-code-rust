use crate::{common::print_by_line::PrintByLine, parse_input};

mod extract;
mod brick;

use extract::*;
use brick::*;

pub fn task() {
    let lines = parse_input!();
    let bricks = extract_input(&lines);

    bricks.print_by_line();
}
