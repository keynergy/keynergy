pub mod analysis;
pub mod keyboard;
pub mod layout;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone, Copy, std::cmp::PartialEq)]
pub struct Pos {
    col: u8,
    row: u8,
}

impl Pos {
    /// Constructs a pos: (col, row) || (x, y)
    pub fn new(col: u8, row: u8) -> Pos {
        Pos { col, row }
    }
}

pub type PosPair = [Pos; 2];

pub type PosGroup = Vec<Pos>;
