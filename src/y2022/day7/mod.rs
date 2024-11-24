use crate::parse_input;

mod filetree;
mod traverse;

use filetree::*;
use traverse::*;

pub fn task() {
    let lines = parse_input!();

    let filetree = traverse(&lines);
    dbg!(&filetree);

    println!("Total size: {}", filetree.size());
}
