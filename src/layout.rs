use crate::Pos;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

pub type Finger = u8;

#[derive(Debug)]
/// Struct that stores basic key data about a layout, as well as its metadata.
pub struct Layout {
    pub name: String,
    pub author: String,
    pub link: String,
    pub year: u32,
    pub keymatrix: Vec<Vec<char>>,
    pub keymap: HashMap<char, Pos>,
    pub anchor: Pos,
}

#[derive(Debug, PartialEq)]
pub enum LayoutError {
    NotFoundError,
    FormatError,
}

impl error::Error for LayoutError {}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LayoutError::NotFoundError => write!(f, "File not found"),
            LayoutError::FormatError => write!(f, "Parsing error"),
        }
    }
}

impl From<io::Error> for LayoutError {
    fn from(_: io::Error) -> Self {
        LayoutError::FormatError
    }
}

/// Reads a layout file and parses it into a Layout.
/// ```rust
/// let l = keynergy::layout::load_layout("testdata/semimak_jq.layout".to_string()).unwrap();
/// assert_eq!(l.name, "Semimak JQ");
/// ```
pub fn load_layout(path: String) -> Result<Layout, LayoutError> {
    // read file
    let file = File::open(path).map_err(|_| LayoutError::NotFoundError)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect::<Vec<_>>();

    // 8 should be the minimum number of lines possible, being a 1 row layout
    if lines.len() < 8 {
        return Err(LayoutError::FormatError);
    }

    // get metadata
    let name = lines[0].as_ref().unwrap();
    let author = lines[1].as_ref().unwrap();
    let link = lines[2].as_ref().unwrap();
    let year = lines[3]
        .as_ref()
        .unwrap()
        .parse::<u32>()
        .map_err(|_| LayoutError::FormatError)?;

    // initialize key structures
    let mut keys: Vec<Vec<char>> = Vec::new();
    let mut keymap: HashMap<char, Pos> = HashMap::new();

    // read layout block
    for l in &lines[5..] {
        let line = l.as_ref().unwrap().trim();
        if line == "" {
            // in .layout files, the end of the layout block is denoted by a blank line
            break;
        }
        keys.push(Vec::new());
        let row = keys.len() - 1;
        let mut col = 0;
        for w in line.split(' ') {
            let c = w.chars().nth(0).unwrap(); // get first char, in case there are more than one
            keys[row].push(c);
            keymap.insert(c, (col, row as u8));
            col += 1;
        }
    }
    let anchorkey = lines[lines.len() - 1]
        .as_ref()
        .unwrap()
        .chars()
        .nth(0)
        .unwrap();
    let anchor = keymap[&anchorkey];
    Ok(Layout {
        name: name.clone(),
        author: author.clone(),
        link: link.clone(),
        year: year.clone(),
        keymatrix: keys,
        keymap: keymap,
        anchor: anchor,
    })
}
