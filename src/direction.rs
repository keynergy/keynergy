use crate::Finger;

#[derive(Debug, Clone, PartialEq)]
/// Defines a direction between two fingers.
pub enum Direction {
    /// Goes from pinky to index.
    Inward,
    /// Goes from index to pinky.
    Outward,
    /// No direction. Usually when the two fingers are the same.
    None,
}

impl Direction {
    /// Returns the Direction from finger a to finger b.
    /// # Example
    /// ```rust
    /// use keynergy::{Finger, Hand, FingerKind, Direction};
    /// let ri = Finger::new(Hand::Right, FingerKind::Index);
    /// let rm = Finger::new(Hand::Right, FingerKind::Middle);
    /// assert_eq!(Direction::Inward, Direction::from(rm, ri));
    /// assert_eq!(Direction::Outward, Direction::from(ri, rm));
    ///```
    pub fn from(a: Finger, b: Finger) -> Direction {
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
}
