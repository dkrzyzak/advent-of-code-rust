use std::fmt::Debug;

pub trait PrintByLine {
    fn print_by_line(&self);
}

impl<T: Debug> PrintByLine for Vec<T> {
    fn print_by_line(&self) {
        for item in self.iter() {
            println!("{:?}", item);
        }
    }
}
