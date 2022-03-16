use crate::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Finger {
    pub hand: Hand,
    pub kind: FingerKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FingerKind {
    Thumb,
    Index,
    Middle,
    Ring,
    Pinky,
}

impl Finger {
    pub fn new(hand: Hand, kind: FingerKind) -> Finger {
        Finger { hand, kind }
    }
    /// Returns the direction between self and the argument f.
    /// ```rust
    /// use keynergy::{Finger, Hand, FingerKind, Direction};
    /// let ri = Finger::new(Hand::Right, FingerKind::Index);
    /// let rm = Finger::new(Hand::Right, FingerKind::Middle);
    /// assert_eq!(Direction::Outward, ri.dir_to(rm));
    /// assert_eq!(Direction::Inward, rm.dir_to(ri));
    /// assert_eq!(Direction::None, ri.dir_to(ri));
    /// ```
    pub fn dir_to(self, f: Finger) -> Direction {
        Direction::from(self, f)
    }
}
