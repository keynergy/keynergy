//use std::collections::HashMap;

use crate::keyboard::{Finger, Keyboard};
//use crate::layout::{self, Layout};
//use crate::Pos;
//use rhai::{Engine, EvalAltResult, INT};

#[derive(Debug, Clone)]
pub struct Metric {
    name: String,
    count: u64,
    amount: f64,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Inward,
    Outward,
    None,
}

pub fn direction(a: Finger, b: Finger) -> Direction {
    if b < a {
        Direction::Inward
    } else if a < b {
        Direction::Outward
    } else {
        Direction::None
    }
}

impl Keyboard {
    /*pub fn xdist(&self, a: &Pos, b: &Pos) -> f64 {

}*/
}
