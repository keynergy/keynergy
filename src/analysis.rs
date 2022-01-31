use crate::keyboard::Finger;

#[derive(Debug, Clone)]
pub struct Metric {
    name: String,
    count: u64,
    amount: f64,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Inward,
    Outward,
    None,
}

pub fn direction(a: Finger, b: Finger) -> Direction {
    if a.hand != b.hand {
        Direction::None
    } else {
        use std::cmp::Ordering::*;
        match a.kind.cmp(&b.kind) {
            Less => Direction::Outward,
            Equal => Direction::None,
            Greater => Direction::Inward,
        }
    }
}
