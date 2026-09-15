#[derive(Debug, serde::Deserialize)]
pub struct Coordinates {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}

impl Coordinates {
    pub const fn contains(&self, x: i16, y: i16) -> bool {
        (if self.x1 == -1 { true } else { self.x1 <= x })
            && if self.x2 == -1 { true } else { x <= self.x2 }
            && if self.y1 == -1 { true } else { self.y1 <= y }
            && if self.y2 == -1 { true } else { y <= self.y2 }
    }
}
