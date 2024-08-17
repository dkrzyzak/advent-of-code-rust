pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn lcm_vec(numbers: Vec<u64>) -> u64 {
    numbers.into_iter().fold(1, |a, b| lcm(a, b))
}

pub trait Between {
    fn between(&self, a: usize, b: usize) -> bool;
}

impl Between for usize {
    fn between(&self, a: usize, b: usize) -> bool {
        *self > a && *self < b
    }
}
