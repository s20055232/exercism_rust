use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let word_chars = get_sort_chars(&lower_word);

    // solution 1
    possible_anagrams
        .iter()
        .filter(|w| {
            let possible = w.to_lowercase();
            possible != lower_word && get_sort_chars(&possible) == word_chars
        })
        .copied()
        .collect()

    // solution 2
    // let result = HashSet::new();
    // for &w in possible_anagrams {
    //     let possible_word_lower = w.to_lowercase();
    //     if possible_word_lower == lower_word {
    //         continue;
    //     }
    //     let tmp = get_sort_chars(&possible_word_lower);

    //     if word_chars == tmp {
    //         result.insert(w);
    //     }
    // }
    // result
}

fn get_sort_chars(s: &str) -> Vec<char> {
    let mut sort_string = s.chars().collect::<Vec<char>>();
    sort_string.sort_unstable();
    sort_string
}
