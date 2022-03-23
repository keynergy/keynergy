use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
/// A position on a keyboard or layout (col, row)
pub struct Pos {
    pub col: usize,
    pub row: usize,
}

impl Pos {
    /// Constructs a pos: (col, row) || (x, y)
    pub fn new(col: usize, row: usize) -> Pos {
        Pos { col, row }
    }
}

pub type PosPair = [Pos; 2];
