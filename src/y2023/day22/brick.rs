#[derive(Debug)]
pub struct Brick {
    pub x1: isize,
    pub x2: isize,
    pub y1: isize,
    pub y2: isize,
    pub z1: isize,
    pub z2: isize,
}

impl Brick {
    pub fn new(x1: isize, x2: isize, y1: isize, y2: isize, z1: isize, z2: isize) -> Self {
        Brick { x1, x2, y1, y2, z1, z2 }
    }
}