use crate::fingers::*;
use crate::{Finger, Pos};

/// Describes a physical keyboard and its properties.
#[derive(Clone)]
pub struct Keyboard {
    pub name: String,
    /// how staggered each row is, in units
    pub rowstagger: Vec<f64>,
    /// how staggered each column is, in units
    pub colstagger: Vec<f64>,
    /// number of (cols, rows)
    pub dimensions: [usize; 2],
    /// how tall each key is, in units
    pub keyheight: f64,
    pub fingers: Vec<Vec<Finger>>,
}

impl Keyboard {
    /// Returns the horizontal distance between two keys
    pub fn xdist(&self, a: &Pos, b: &Pos) -> f64 {
        ((self.rowstagger[a.row as usize] + a.col as f64)
            - (self.rowstagger[b.row as usize] + b.col as f64))
            .abs()
    }
    /// Returns the vertical distance between two keys
    pub fn ydist(&self, a: &Pos, b: &Pos) -> f64 {
        ((self.colstagger[a.col as usize] + a.row as f64)
            - (self.colstagger[b.col as usize] + b.row as f64))
            .abs()
    }
    pub fn matrix() -> Self {
        Keyboard {
            name: "Matrix".to_string(),
            rowstagger: vec![0.0, 0.0, 0.0],
            colstagger: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            dimensions: [10, 3],
            keyheight: 0.5,
            fingers: vec![
                vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
                vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
                vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Keyboard, Pos};

    #[test]
    fn distance() {
        let matrix = Keyboard::matrix();
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
    #[test]
    fn direction() {
        use crate::{Direction, Finger, FingerKind, Hand};
        let ri = Finger::new(Hand::Right, FingerKind::Index);
        let rm = Finger::new(Hand::Right, FingerKind::Middle);
        let li = Finger::new(Hand::Left, FingerKind::Index);
        assert_eq!(Direction::Inward, Direction::from(rm, ri));
        assert_eq!(Direction::Outward, Direction::from(ri, rm));
        assert_eq!(Direction::None, Direction::from(ri, li));
        assert_eq!(Direction::None, Direction::from(ri, ri));
    }
}
