use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let mut word_chars: Vec<char> = lower_word.chars().collect();
    word_chars.sort_unstable();

    let mut result = HashSet::new();
    for &w in possible_anagrams {
        let possible_word_lower = w.to_lowercase();
        if possible_word_lower == lower_word {
            continue;
        }
        let mut tmp: Vec<char> = possible_word_lower.chars().collect();
        tmp.sort_unstable();

        if word_chars == tmp {
            result.insert(w);
        }
    }
    result
}
