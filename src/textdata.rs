use std::collections::HashMap;

pub struct TextData {
    pub chars: HashMap<char, u64>,
    pub bigrams: HashMap<[char; 2], u64>,
    pub trigrams: HashMap<[char; 3], u64>,
    pub skip_1_grams: HashMap<[char; 2], u64>,
}

impl TextData {
    pub fn from(text: String) -> Self {
        let mut chars: HashMap<char, u64> = HashMap::with_capacity(30);
        let mut bigrams: HashMap<[char; 2], u64> = HashMap::with_capacity(30 * 30);
        let mut trigrams: HashMap<[char; 3], u64> = HashMap::with_capacity(30 * 30 * 15);
        let mut skip_1_grams: HashMap<[char; 2], u64> = HashMap::with_capacity(30 * 30);
        for v in text
            .chars()
            .map(|x| x.to_ascii_lowercase())
            .collect::<Vec<char>>()
            .windows(3)
        {
            let ch = chars.entry(v[0]).or_insert(0);
            *ch += 1;
            if v.len() >= 2 {
                let bg = bigrams.entry([v[0], v[1]]).or_insert(0);
                *bg += 1;
            }
            if v.len() == 3 {
                let tg = trigrams.entry([v[0], v[1], v[2]]).or_insert(0);
                let sg = skip_1_grams.entry([v[0], v[1]]).or_insert(0);
                *tg += 1;
                *sg += 1;
            }
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
        let data = TextData::from("Hello world!".to_string());
        assert_eq!(*data.chars.get(&'l').unwrap(), 3);
        assert_eq!(*data.bigrams.get(&['l', 'd']).unwrap(), 1);
        assert_eq!(*data.trigrams.get(&['w', 'o', 'r']).unwrap(), 1);
    }
}
