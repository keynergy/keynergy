use crate::Pos;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Finger {
    pub hand: Hand,
    pub kind: FingerKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FingerKind {
    Thumb,
    Index,
    Middle,
    Ring,
    Pinky,
}

pub struct Fingermap {
    pub matrix: Vec<Vec<Finger>>,
    pub map: HashMap<Finger, Pos>,
}

/// Describes a physical keyboard and its properties.
pub struct Keyboard {
    pub name: String,
    /// how staggered each row is, in cm
    pub rowstagger: Vec<f32>,
    /// how staggered each column is, in cm
    pub colstagger: Vec<f32>,
    /// number of (cols, rows)
    pub dimensions: [u8; 2],
    /// how tall each key is, in cm
    pub keyheight: f64,
    /// how wide each key is, in cm
    pub keywidth: f64,
    pub fingermap: Fingermap,
    pub anchor: Pos,
}
