use std::collections::HashMap;

pub fn is_english_text(text: &str) -> bool {
    let score = english_score(text);
    score > 70.0
}

pub fn english_score(text: &str) -> f64 {
    let expected_frequencies = HashMap::from([
        (' ', 13.0000),
        ('e', 11.1607),
        ('a', 8.4966),
        ('r', 7.5809),
        ('i', 7.5448),
        ('o', 7.1635),
        ('t', 6.9509),
        ('n', 6.6544),
        ('s', 5.7351),
        ('l', 5.4893),
        ('c', 4.5388),
        ('u', 3.6308),
        ('d', 3.3844),
        ('p', 3.1671),
        ('m', 3.0129),
        ('h', 3.0034),
        ('g', 2.4705),
        ('b', 2.0720),
        ('f', 1.8121),
        ('y', 1.7779),
        ('w', 1.2899),
        ('k', 1.1016),
        ('v', 1.0074),
        ('x', 0.2902),
        ('z', 0.2722),
        ('j', 0.1965),
        ('q', 0.1962),
    ]);

    if text.is_empty() {
        return 0.0;
    }

    let mut letter_counter = HashMap::new();
    let mut score = 0.0;

    for c in text
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .map(|c| c.to_ascii_lowercase())
    {
        *letter_counter.entry(c).or_insert(0) += 1;
    }

    for (ch, expected_freq) in expected_frequencies.iter() {
        let count = letter_counter.get(ch).unwrap_or(&0);
        let frequency = *count as f64 * *expected_freq;
        score += frequency / text.len() as f64;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_score_higher_than_japanese() {
        let text = "This is a simple English sentence.";
        let score_en = english_score(text);
        let score_ja = english_score("これは日本語の文章です。");
        assert!(score_en > score_ja);
    }

    #[test]
    fn test_english_score_higher_than_mixed_charts() {
        let text = "This is a simple English sentence.";
        let score_en = english_score(text);
        let mixed_score =
            english_score("これは日本語の文章です。This is a simple English sentence.");
        assert!(score_en > mixed_score);
    }

    #[test]
    fn test_get_english_score_with_empty_string() {
        let text = "";
        let score = english_score(text);
        assert_eq!(score, 0.0);
    }
}
