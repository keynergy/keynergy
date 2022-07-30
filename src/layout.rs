use crate::Pos;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::mem;
use std::path::Path;
use toml;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Formats {
    pub standard: Option<Keys>,
    pub angle: Option<Keys>,
    pub angle_preferred: Option<bool>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
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

impl fmt::Display for Keys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = &self.matrix;
        let mut s = String::new();
        for r in m.iter() {
            for (i, c) in r.iter().enumerate() {
                s.push(*c);
                s.push(' ');
                if i == 4 {
                    s.push(' ');
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Keys {
    pub fn new(matrix: Vec<Vec<char>>, home_row: u8, thumb_row: Option<u8>) -> Self {
        let mut k = Keys {
            matrix,
            map: HashMap::new(),
            home_row,
            thumb_row,
        };
        k.fill_map();
        k
    }
    pub fn fill_map(&mut self) {
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, key) in row.iter().enumerate() {
                self.map.insert(*key, Pos { col: x, row: y });
            }
        }
    }
}

impl Layout {
    /// Reads a layout file and parses it into a Layout.
    /// ```rust
    /// let l = keynergy::Layout::load("testdata/semimak_jq.toml").unwrap();
    /// assert_eq!(l.name, "Semimak JQ");
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Layout, LayoutError> {
        // read file
        let file = fs::read_to_string(path)?;
        let result = toml::from_str(&file);
        let mut layout: Layout = result?;
        // fills the maps for each format if they exist
        [&mut layout.formats.standard, &mut layout.formats.angle].map(|l| {
            if let Some(l) = l.as_mut() {
                l.fill_map()
            }
        });
        if layout.link == Some("".to_string()) {
            layout.link = None;
        }
        Ok(layout)
    }
    pub fn angle_is_preferred(&self) -> bool {
        match self.formats.angle_preferred {
            Some(true) => true,
            _ => false,
        }
    }
}

impl Keys {
    pub fn qwerty() -> Self {
        Keys::new(
            vec![
                vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
                vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
                vec!['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
            ],
            1,
            None,
        )
    }
    /// Returns whether the keys contain a character or not.
    /// # Example
    /// ```rust
    /// use keynergy::{Keys, Pos};
    /// let qwerty = Keys::qwerty();
    /// assert_eq!(qwerty.has_pos(Pos::new(0,0)), true);
    /// assert_eq!(qwerty.has_pos(Pos::new(100,4)), false);
    /// ```
    pub fn has_pos(&self, p: Pos) -> bool {
        #[allow(clippy::collapsible_if)]
        if self.matrix.len() > p.row {
            if self.matrix[p.row].len() > p.col {
                return true;
            }
        }
        false
    }

    /// Returns the character at the given position without checking
    /// if it's valid.
    /// # Examples
    /// ```rust
    /// use keynergy::{Keys, Pos};
    /// let qwerty = Keys::qwerty();
    /// assert_eq!(qwerty.pos_key_unsafe(Pos::new(0,0)), &'q');
    /// ```
    /// This example panics
    /// ```rust,should_panic
    /// use keynergy::{Keys, Pos};
    /// let qwerty = Keys::qwerty();
    /// let c = qwerty.pos_key_unsafe(Pos::new(100, 0));
    /// ```
    pub fn pos_key_unsafe(&self, p: Pos) -> &char {
        &self.matrix[p.row][p.col]
    }

    pub fn pos_key(&self, p: Pos) -> Option<&char> {
        if self.matrix.len() > p.row {
            let row = &self.matrix[p.row];
            if row.len() > p.col {
                return Some(&row[p.col]);
            }
        }
        None
    }

    pub fn swap(&mut self, a: Pos, b: Pos) {
        if a.row == b.row {
            self.matrix[a.row as usize].swap(a.col as usize, b.col as usize)
        } else {
            let gtr: &Pos;
            let lsr: &Pos;
            if a.row > b.row {
                gtr = &a;
                lsr = &b;
            } else {
                gtr = &b;
                lsr = &a;
            }
            let (l, r) = self.matrix.split_at_mut(gtr.row as usize);
            mem::swap(
                &mut l[lsr.row as usize][lsr.col as usize],
                &mut r[(gtr.row - lsr.row - 1) as usize][gtr.col as usize],
            )
        }

        self.map.insert(*self.pos_key_unsafe(a), a);
        self.map.insert(*self.pos_key_unsafe(b), b);
    }
}

#[cfg(test)]
mod tests {
    use crate::Layout;
    use crate::Pos;
    #[test]
    fn load_layout() {
        let semimak_jq = Layout::load("testdata/semimak_jq.toml").unwrap();
        assert_eq!(semimak_jq.name, "Semimak JQ");
        assert_eq!(semimak_jq.author, "semi");
        assert_eq!(
            semimak_jq.link.unwrap(),
            "https://semilin.github.io/semimak"
        );
        assert_eq!(semimak_jq.year, 2021);
        let keys = semimak_jq.formats.standard.unwrap();
        assert_eq!(keys.matrix[0][0], 'f');
        assert_eq!(keys.map[&'l'], Pos::new(1, 0));
        // check that map aligns with matrix
        let mut y = 0;
        for row in &keys.matrix {
            let mut x = 0;
            for key in row {
                assert_eq!(*key, keys.matrix[y][x]);
                x += 1;
            }
            y += 1;
        }
    }
    #[test]
    fn keys_swap() {
        let semimak_jq = Layout::load("testdata/semimak_jq.toml").unwrap();
        let mut keys = semimak_jq.formats.standard.unwrap();
        keys.swap(Pos::new(0, 0), Pos::new(1, 0));
        assert_eq!(keys.matrix[0][0], 'l');
        assert_eq!(keys.matrix[0][1], 'f');
        assert_eq!(keys.map[&'l'], Pos::new(0, 0));
        assert_eq!(keys.map[&'f'], Pos::new(1, 0));

        keys.swap(Pos::new(3, 0), Pos::new(2, 1));
        assert_eq!(keys.matrix[0][3], 'n');
        assert_eq!(keys.matrix[1][2], 'v');
        assert_eq!(keys.map[&'n'], Pos::new(3, 0));
        assert_eq!(keys.map[&'v'], Pos::new(2, 1));
    }
}
