use std::{collections::HashMap, fmt::Debug};

pub trait PrintByLine {
    fn print_by_line(&self);
}

impl<T: Debug> PrintByLine for Vec<T> {
    fn print_by_line(&self) {
        for item in self.iter() {
            println!("{:?}", item);
        }
        println!("");
    }
}

impl<K: Debug, V: Debug> PrintByLine for HashMap<K, V> {
    fn print_by_line(&self) {
        for item in self.iter() {
            println!("{:?}: {:?}", item.0, item.1);
        }
        println!("");
    }
}