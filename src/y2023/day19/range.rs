#[derive(Debug, Clone)]
pub struct PartRange {
    pub x: Range,
    pub m: Range,
    pub a: Range,
    pub s: Range,
}

impl PartRange {
    pub fn new() -> PartRange {
        PartRange {
            x: Range::new(),
            m: Range::new(),
            a: Range::new(),
            s: Range::new(),
        }
    }

    pub fn total(&self) -> usize {
        self.x.total() + self.m.total() + self.a.total() + self.s.total()
    }

    pub fn split(&self, field_name: &str, at: usize, operation: &str) -> (PartRange, PartRange) {
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
    pub bounds: Vec<usize>,
}

impl Range {
    pub fn new() -> Range {
        Range {
            bounds: vec![1, 4000],
        }
    }

    // x > 300: (low, 300), (300 + 1, high)
    // x < 300: (low, 300 - 1), (300, high)

    pub fn split(&self, at: usize, operator: &str) -> (Range, Range) /* (ok, not_ok) */ {
        let split_index = self.bounds.iter().position(|bound| *bound > at).unwrap();
        let mut bounds_low = (&self.bounds[..split_index]).to_vec();
        let mut bounds_up = (&self.bounds[split_index..]).to_vec();

        // println!(
        //     "{:?}, split index: {}, {:?}, {:?}",
        //     self.bounds, split_index, bounds_low, bounds_up
        // );

        if operator == ">" {
            bounds_low.push(at);
            bounds_up.insert(0, at + 1);
        
            return (Range { bounds: bounds_up }, Range { bounds: bounds_low });
        } else {
            bounds_low.push(at - 1);
            bounds_up.insert(0, at);
            (Range { bounds: bounds_low }, Range { bounds: bounds_up })
        }
    }

    pub fn total(&self) -> usize {
        let mut sum = 0;
        for i in (0..self.bounds.len()).step_by(2) {
            let lower = self.bounds[i];
            let upper = self.bounds[i + 1];

            sum += upper - lower + 1;
        }

        sum
    }
}
