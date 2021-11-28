pub mod layout;
pub mod keyboard;

/// Represents (x, y) or (col, row)
pub type Pos = (u8, u8);

#[cfg(test)]
mod tests {
    #[test]
    fn load_layout_exists() {
        let semimak_jq =
            crate::layout::load_layout("testdata/semimak_jq.layout".to_string()).unwrap();
        assert_eq!(semimak_jq.keymatrix[0][0], 'f');
        assert_eq!(semimak_jq.keymap[&'l'], (1, 0));
        assert_eq!(semimak_jq.anchor, (0, 1));
        let mut y = 0;
        for row in &semimak_jq.keymatrix {
            let mut x = 0;
            for key in row {
                assert_eq!(*key, semimak_jq.keymatrix[y][x]);
                x += 1;
            }
            y += 1;
        }
    }
}
