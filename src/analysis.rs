use crate::keyboard::{self, Finger, Keyboard};
use crate::layout::{self, Layout};
use crate::Pos;

pub enum Direction {
  Inward,
  Outward,
  None,
}

pub fn distance(a: Pos, b: Pos) -> f64 {
  let yd: f64 = ((a.row as i8 - b.row as i8).abs()).into();
  let xd: f64 = ((a.col as i8 - b.col as i8).abs()).into();

  ((yd+xd)*(yd+xd)).sqrt()
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
