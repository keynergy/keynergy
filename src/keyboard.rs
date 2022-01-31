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

#[derive(Clone)]
pub struct Fingermap {
    pub matrix: Vec<Vec<Finger>>,
    pub map: HashMap<Finger, Pos>,
}

/// Describes a physical keyboard and its properties.
#[derive(Clone)]
pub struct Keyboard {
    pub name: String,
    /// how staggered each row is, in units
    pub rowstagger: Vec<f64>,
    /// how staggered each column is, in units
    pub colstagger: Vec<f64>,
    /// number of (cols, rows)
    pub dimensions: [u8; 2],
    /// how tall each key is, in units
    pub keyheight: f64,
    /// how wide each key is, in units
    pub keywidth: f64,
    pub fingers: Fingermap,
}

impl Keyboard {
    pub fn xdist(&self, a: &Pos, b: &Pos) -> f64 {
        ((self.rowstagger[a.row as usize] + a.col as f64)
            - (self.rowstagger[b.row as usize] + b.col as f64))
            .abs()
    }
    pub fn ydist(&self, a: &Pos, b: &Pos) -> f64 {
        ((self.colstagger[a.col as usize] + a.row as f64)
            - (self.colstagger[b.col as usize] + b.row as f64))
            .abs()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        keyboard::{Fingermap, Keyboard},
        Pos,
    };
    use std::collections::HashMap;

    #[test]
    fn distance() {
        let matrix = Keyboard {
            name: "Matrix".to_string(),
            rowstagger: vec![0.0, 0.0, 0.0],
            colstagger: vec![0.0, 0.0, 0.0],
            dimensions: [10, 3],
            keyheight: 1.0,
            keywidth: 1.0,
            fingers: Fingermap {
                matrix: vec![vec![]],
                map: HashMap::new(),
            },
        };
        assert_eq!(matrix.xdist(&Pos::new(0, 0), &Pos::new(1, 0)), 1.0);
        // shouldn't have any horizontal distance
        assert_eq!(matrix.xdist(&Pos::new(0, 0), &Pos::new(0, 1)), 0.0);

        assert_eq!(matrix.ydist(&Pos::new(0, 0), &Pos::new(0, 2)), 2.0);
        // shouldn't have any vertical distance
        assert_eq!(matrix.ydist(&Pos::new(0, 0), &Pos::new(2, 0)), 0.0);

        let mut ansi = matrix.clone();
        ansi.rowstagger = vec![-0.25, 0.0, 0.5];
        assert_eq!(ansi.xdist(&Pos::new(0, 0), &Pos::new(1, 0)), 1.0);
        assert_eq!(ansi.xdist(&Pos::new(0, 1), &Pos::new(1, 1)), 1.0);
        assert_eq!(ansi.xdist(&Pos::new(0, 2), &Pos::new(1, 2)), 1.0);

        assert_eq!(ansi.xdist(&Pos::new(0, 0), &Pos::new(1, 1)), 1.25);
        assert_eq!(ansi.xdist(&Pos::new(0, 0), &Pos::new(1, 2)), 1.75);
    }
}
