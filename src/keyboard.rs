use crate::layout;
use crate::Pos;

/// describes a physical keyboard and its properties
pub struct Keyboard {
    pub name: String,
    /// how staggered each row is, in cm
    pub rowstagger: Vec<f32>,
    /// how staggered each column is, in cm
    pub colstagger: Vec<f32>,
    /// number of (cols, rows)
    pub dimensions: (u8, u8),
    /// how tall each key is, in cm
    pub keyheight: f64,
}
