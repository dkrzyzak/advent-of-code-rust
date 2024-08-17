#[derive(Debug)]
pub struct Range {
    pub destination_start: u64,
    pub source_start: u64,
    pub range_len: u64,
}

impl Range {
    pub fn includes(&self, value: u64) -> Option<u64> {
        if value < self.source_start || value >= self.source_start + self.range_len {
            return None;
        }
        

        let offset = value - self.source_start;
        let destination = self.destination_start + offset;

        Some(destination)
    }
}

// pub enum 

#[derive(Debug)]
pub struct Mapping {
    pub source: String,
    pub destination: String,
    pub ranges: Vec<Range>,
}

impl Mapping {
    pub fn get_destination(&self, value: u64) -> u64 {
        for range in self.ranges.iter() {
            if let Some(destination) = range.includes(value) {
                return destination;
            }
        }

        return value;
    }
}
