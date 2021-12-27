pub mod analysis;
pub mod keyboard;
pub mod layout;

#[derive(Default, Debug, Clone, Copy, std::cmp::PartialEq)]
pub struct Pos {
    col: u8,
    row: u8,
}

impl Pos {
    pub fn new(col: u8, row: u8) -> Pos {
        Pos { col, row }
    }
}

pub type PosPair = [Pos; 2];

pub type PosGroup = Vec<Pos>;

#[cfg(test)]
mod layout_tests {
    use crate::layout::Layout;
    use crate::Pos;
    #[test]
    fn load_layout() {
        let semimak_jq = Layout::load("testdata/semimak_jq.layout".to_string()).unwrap();
        assert_eq!(semimak_jq.name, "Semimak JQ");
        assert_eq!(semimak_jq.author, "semi");
        assert_eq!(semimak_jq.link, "https://semilin.github.io/semimak");
        assert_eq!(semimak_jq.year, 2021);
        assert_eq!(semimak_jq.keys.matrix[0][0], 'f');
        assert_eq!(semimak_jq.keys.map[&'l'], Pos::new(1, 0));
        assert_eq!(semimak_jq.anchor, Pos::new(0, 1));
	// check that map aligns with matrix
        let mut y = 0;
        for row in &semimak_jq.keys.matrix {
            let mut x = 0;
            for key in row {
                assert_eq!(*key, semimak_jq.keys.matrix[y][x]);
                x += 1;
            }
            y += 1;
        }
    }
    #[test]
    fn keys_swap() {
        let mut semimak_jq = Layout::load("testdata/semimak_jq.layout".to_string()).unwrap();
        semimak_jq.keys.swap(&[Pos::new(0, 0), Pos::new(1, 0)]);
        assert_eq!(semimak_jq.keys.matrix[0][0], 'l');
        assert_eq!(semimak_jq.keys.matrix[0][1], 'f');
        assert_eq!(semimak_jq.keys.map[&'l'], Pos::new(0, 0));
        assert_eq!(semimak_jq.keys.map[&'f'], Pos::new(1, 0));

        semimak_jq.keys.swap(&[Pos::new(3, 0), Pos::new(2, 1)]);
        assert_eq!(semimak_jq.keys.matrix[0][3], 'n');
        assert_eq!(semimak_jq.keys.matrix[1][2], 'v');
        assert_eq!(semimak_jq.keys.map[&'n'], Pos::new(3, 0));
        assert_eq!(semimak_jq.keys.map[&'v'], Pos::new(2, 1));
    }
}
