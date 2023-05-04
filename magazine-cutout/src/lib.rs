// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = HashMap::new();
    // 先建立字典，知道有哪些字，出現次數多少
    for i in magazine {
        words
            .entry(*i)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    // 消耗字數庫存，如果小於0則false，大於則true
    for i in note {
        match words.get(*i) {
            Some(x) => {
                let value = &x.clone() - 1;
                if value < 0 {
                    return false;
                } else {
                    words.insert(*i, value);
                }
            }
            None => return false,
        }
    }
    true
}
