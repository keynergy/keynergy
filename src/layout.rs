use crate::Pos;
use crate::PosPair;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::mem;

pub type Finger = u8;

#[derive(Debug)]
/// Wraps key data with the layout's metadata.
pub struct Layout {
    pub name: String,
    pub author: String,
    pub link: String,
    pub year: u32,
    pub keys: Keys,
    pub anchor: Pos,
}

#[derive(Debug)]
pub struct Keys {
    pub matrix: Vec<Vec<char>>,
    pub map: HashMap<char, Pos>,
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

impl Layout {
    /// Reads a layout file and parses it into a Layout.
    /// ```rust
    /// let l = keynergy::layout::Layout::load("testdata/semimak_jq.layout".to_string()).unwrap();
    /// assert_eq!(l.name, "Semimak JQ");
    /// ```
    pub fn load(path: String) -> Result<Layout, LayoutError> {
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
            if line.is_empty() {
                // in .layout files, the end of the layout block is denoted by a blank line
                break;
            }
            keys.push(Vec::new());
            let row = keys.len() - 1;
            for (col, w) in line.split(' ').enumerate() {
                let c = w.chars().next().unwrap(); // get first char, in case there are more than one
		keys[row].push(c);
                keymap.insert(c, Pos{col: col as u8, row: row as u8});	
            }
        }
        let anchorkey = lines[lines.len() - 1]
            .as_ref()
            .unwrap()
            .chars()
            .next()
	    .unwrap();
	let anchor = keymap[&anchorkey];
	Ok(Layout {
	    name: name.clone(),
	    author: author.clone(),
	    link: link.clone(),
	    year,
	    keys: Keys {
		matrix: keys,
		map: keymap,
	    },
	    anchor
	})
    }
}

impl Keys {
    pub fn has_pos(&self, p: Pos) -> bool {
	#[allow(clippy::collapsible_if)]
	if self.matrix.len() > p.row as usize {
	    if self.matrix[p.row as usize].len() > p.col as usize {
		return true
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
	    mem::swap(&mut l[lsr.row as usize][lsr.col as usize], &mut r[(gtr.row - lsr.row - 1) as usize][gtr.col as usize])
	}

	self.map.insert(*self.pos_key(p[0]), p[0]);
	self.map.insert(*self.pos_key(p[1]), p[1]);
    }
}

