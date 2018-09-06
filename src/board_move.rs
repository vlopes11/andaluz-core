use std::cmp::Ordering;
use square::Square;

pub struct BoardMove {
    pub s: Square,
    pub h: f64,
}

impl PartialEq for BoardMove {
    fn eq(&self, other: &BoardMove) -> bool {
        self.s.x == other.s.x && self.s.y == other.s.y
    }
}

impl PartialOrd for BoardMove {
    fn partial_cmp(&self, other: &BoardMove) -> Option<Ordering> {
        self.h.partial_cmp(&other.h)
    }
}

impl Eq for BoardMove {}

impl Ord for BoardMove {
    fn cmp(&self, other: &BoardMove) -> Ordering {
        match self.partial_cmp(other) {
            Some(m) => m,
            None => Ordering::Equal,
        }
    }
}
