pub trait Between {
    fn between(&self, a: usize, b: usize) -> bool;
}

impl Between for usize {
    fn between(&self, a: usize, b: usize) -> bool {
        *self > a && *self < b
    }
}
