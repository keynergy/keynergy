pub mod layout;
// Pos represents (x, y) or (col, row)
pub type Pos = (u8, u8);

#[cfg(test)]
mod tests {
    #[test]
    fn load_layout_test() {
	let semimak_jq = crate::layout::load_layout("testdata/semimak_jq.layout".to_string()).unwrap();
	assert_eq!(semimak_jq.keymatrix[0][0], 'f');
    }
}
