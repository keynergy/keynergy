use crate::Pos;
use crate::PosPair;
use serde::Deserialize;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::mem;
use toml;

#[derive(Deserialize, Debug)]
pub struct Formats {
    pub standard: Option<Keys>,
    pub angle: Option<Keys>,
}

#[derive(Deserialize, Debug)]
/// Wraps key data with the layout's metadata.
pub struct Layout {
    pub name: String,
    /// Name of the creator of the layout
    pub author: String,
    /// Link to the layout's web page
    pub link: Option<String>,
    /// Year that the layout was released in
    pub year: u32,
    pub formats: Formats,
}

#[derive(Deserialize, Debug)]
pub struct Keys {
    pub matrix: Vec<Vec<char>>,
    pub map: HashMap<char, Pos>,
    pub home_row: u8,
    pub thumb_row: Option<u8>,
}

#[derive(Debug, PartialEq)]
pub enum LayoutError {
    FileError,
    TomlError,
}

impl error::Error for LayoutError {}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LayoutError::FileError => write!(f, "Parsing error"),
            LayoutError::TomlError => write!(f, "Parsing error"),
        }
    }
}

impl From<toml::de::Error> for LayoutError {
    fn from(_: toml::de::Error) -> Self {
        LayoutError::TomlError
    }
}

impl From<io::Error> for LayoutError {
    fn from(_: io::Error) -> Self {
        LayoutError::FileError
    }
}

impl Keys {
    pub fn fill_map(&mut self) {
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, key) in row.iter().enumerate() {
                self.map.insert(
                    *key,
                    Pos {
                        col: x as u8,
                        row: y as u8,
                    },
                );
            }
        }
    }
}

impl Layout {
    /// Reads a layout file and parses it into a Layout.
    /// ```rust
    /// let l = keynergy::layout::Layout::load("testdata/semimak_jq.toml".to_string()).unwrap();
    /// assert_eq!(l.name, "Semimak JQ");
    /// ```
    pub fn load(path: String) -> Result<Layout, LayoutError> {
        // read file
        let file = fs::read_to_string(path)?;
        let result = toml::from_str(&file);
        let mut layout: Layout = result?;
        // fills the maps for each format if they exist
        // TODO make this less redundant
        layout.formats.standard.as_mut().map(|l| l.fill_map());
        layout.formats.angle.as_mut().map(|l| l.fill_map());
        Ok(layout)
    }
}

impl Keys {
    pub fn has_pos(&self, p: Pos) -> bool {
        #[allow(clippy::collapsible_if)]
        if self.matrix.len() > p.row as usize {
            if self.matrix[p.row as usize].len() > p.col as usize {
                return true;
            }
        }
        false
    }

    pub fn pos_key(&self, p: Pos) -> &char {
        &self.matrix[p.row as usize][p.col as usize]
    }

    pub fn swap(&mut self, p: &PosPair) {
        if p[0].row == p[1].row {
            self.matrix[p[0].row as usize].swap(p[0].col as usize, p[1].col as usize)
        } else {
            let gtr: &Pos;
            let lsr: &Pos;
            if p[0].row > p[1].row {
                gtr = &p[0];
                lsr = &p[1];
            } else {
                gtr = &p[1];
                lsr = &p[0];
            }
            let (l, r) = self.matrix.split_at_mut(gtr.row as usize);
            mem::swap(
                &mut l[lsr.row as usize][lsr.col as usize],
                &mut r[(gtr.row - lsr.row - 1) as usize][gtr.col as usize],
            )
        }

        self.map.insert(*self.pos_key(p[0]), p[0]);
        self.map.insert(*self.pos_key(p[1]), p[1]);
    }
}
