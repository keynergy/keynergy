use crate::{Finger, Hand, FingerKind};

pub const LT: Finger = Finger {
    hand: Hand::Left,
    kind: FingerKind::Thumb,
};
pub const LI: Finger = Finger {
    hand: Hand::Left,
    kind: FingerKind::Index,
};
pub const LM: Finger = Finger {
    hand: Hand::Left,
    kind: FingerKind::Middle,
};
pub const LR: Finger = Finger {
    hand: Hand::Left,
    kind: FingerKind::Ring,
};
pub const LP: Finger = Finger {
    hand: Hand::Left,
    kind: FingerKind::Pinky,
};

pub const RT: Finger = Finger {
    hand: Hand::Right,
    kind: FingerKind::Thumb,
};
pub const RI: Finger = Finger {
    hand: Hand::Right,
    kind: FingerKind::Index,
};
pub const RM: Finger = Finger {
    hand: Hand::Right,
    kind: FingerKind::Middle,
};
pub const RR: Finger = Finger {
    hand: Hand::Right,
    kind: FingerKind::Ring,
};
pub const RP: Finger = Finger {
    hand: Hand::Right,
    kind: FingerKind::Pinky,
};
