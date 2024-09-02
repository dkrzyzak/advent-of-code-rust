use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct PartRange {
    pub x: Range,
    pub m: Range,
    pub a: Range,
    pub s: Range,
}

impl PartRange {
    pub fn new() -> Self {
        PartRange {
            x: Range::default(),
            m: Range::default(),
            a: Range::default(),
            s: Range::default(),
        }
    }

    pub fn total(&self) -> usize {
        self.x.total() * self.m.total() * self.a.total() * self.s.total()
    }

    pub fn split(&self, field_name: &str, at: usize, operation: &str) -> (Self, Self) {
        let bounds = match field_name {
            "x" => &self.x,
            "m" => &self.m,
            "a" => &self.a,
            "s" => &self.s,
            _ => unreachable!("Wrong letter!"),
        };

        let (range_ok, range_bad) = bounds.split(at, operation);

        match field_name {
            "x" => (
                PartRange { x: range_ok, ..self.clone() },
                PartRange { x: range_bad, ..self.clone() },
            ), 
            "m" => (
                PartRange { m: range_ok, ..self.clone() },
                PartRange { m: range_bad, ..self.clone() },
            ),
            "a" => (
                PartRange { a: range_ok, ..self.clone() },
                PartRange { a: range_bad, ..self.clone() },
            ),
            "s" => (
                PartRange { s: range_ok, ..self.clone() },
                PartRange { s: range_bad, ..self.clone() },
            ),
            _ => unreachable!("Wrong letter!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Range {
    pub lower: usize,
    pub upper: usize,
}

impl Range {
    pub fn default() -> Self {
        Range {
            lower: 1,
            upper: 4000,
        }
    }

    pub fn new(lower: usize, upper: usize) -> Self {
        Range { upper, lower }
    }

    // x > 300: (low, 300), (300 + 1, high)
    // x < 300: (low, 300 - 1), (300, high)
    pub fn split(&self, at: usize, operator: &str) -> (Range, Range) /* (ok, not_ok) */ {
        if operator == ">" {
            return (Range::new(at + 1, self.upper), Range::new(self.lower, at));
        } else {
            return (Range::new(self.lower, at - 1), Range::new(at, self.upper));
        }
    }

    pub fn total(&self) -> usize {
        self.upper - self.lower + 1
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.lower, self.upper)
    }
}

impl Display for PartRange {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "x: {}, m: {}, a: {}, s: {}", self.x, self.m, self.a, self.s)
    }
}
