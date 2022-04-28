use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TextData {
    pub chars: HashMap<char, u64>,
    pub bigrams: HashMap<[char; 2], u64>,
    pub trigrams: HashMap<[char; 3], u64>,
    pub skip_1_grams: HashMap<[char; 2], u64>,
}

impl TextData {
    pub fn from(text: &str) -> Self {
        let mut chars: HashMap<char, u64> = HashMap::with_capacity(30);
        let mut bigrams: HashMap<[char; 2], u64> = HashMap::with_capacity(30 * 30);
        let mut trigrams: HashMap<[char; 3], u64> = HashMap::with_capacity(30 * 30 * 15);
        let mut skip_1_grams: HashMap<[char; 2], u64> = HashMap::with_capacity(30 * 30);
        let mut iter = text.chars().map(|x| x.to_ascii_lowercase());
        if let (Some(mut c1), Some(mut c2)) = (iter.next(), iter.next()) {
            for c3 in iter {
                *chars.entry(c1).or_insert(0) += 1;
                *bigrams.entry([c1, c2]).or_insert(0) += 1;
                *skip_1_grams.entry([c1, c3]).or_insert(0) += 1;
                *trigrams.entry([c1, c2, c3]).or_insert(0) += 1;
                c1 = c2;
                c2 = c3;
            }
            *chars.entry(c1).or_insert(0) += 1;
            *chars.entry(c2).or_insert(0) += 1;
            *bigrams.entry([c1, c2]).or_insert(0) += 1;
        }
        Self {
            chars,
            bigrams,
            trigrams,
            skip_1_grams,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TextData;

    #[test]
    fn get_text_data() {
        let data = TextData::from("Hello world!");
        assert_eq!(*data.chars.get(&'l').unwrap(), 3);
        assert_eq!(*data.bigrams.get(&['l', 'd']).unwrap(), 1);
        assert_eq!(*data.trigrams.get(&['w', 'o', 'r']).unwrap(), 1);
	assert_eq!(*data.skip_1_grams.get(&['w', 'r']).unwrap(), 1)
    }
}
