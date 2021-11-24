use std::collections::HashMap;

pub type Finger = u8;

pub struct Layout<'a> {
    keymatrix: &'a Vec<Vec<char>>,
    keymap: &'a HashMap<char, Pos>,
}
