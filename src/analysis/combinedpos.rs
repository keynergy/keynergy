use crate::{fingers::*, Keyboard, Pos};
use ketos_derive::{ForeignValue, FromValueClone, IntoValue, StructValue};
#[derive(Clone, Debug, IntoValue, FromValueClone, ForeignValue, StructValue)]
/// A structure that combines the finger that is used and the actual
/// position data, used for Ketos metric functions
pub struct CombinedPos {
    x: f64,
    y: f64,
    finger: u8,
}

pub type CombinedPosGroup = Vec<CombinedPos>;

impl CombinedPos {
    pub fn from(kb: &Keyboard, p: Pos) -> Self {
        Self {
            x: kb.rowstagger[p.row] + p.col as f64,
            y: kb.colstagger[p.col] + p.row as f64,
            finger: match kb.fingers[p.row][p.col] {
                LP => 0,
                LR => 1,
                LM => 2,
                LI => 3,
                LT => 4,

                RP => 9,
                RR => 8,
                RM => 7,
                RI => 6,
                RT => 5,
            },
        }
    }
    /// creates a CombinedPosGroup from a Vector of Positions
    pub fn from_group(kb: &Keyboard, p: Vec<&Pos>) -> CombinedPosGroup {
        p.iter()
            .map(|x| CombinedPos::from(&kb, **x))
            .collect::<CombinedPosGroup>()
    }
}
