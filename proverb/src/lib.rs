pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let result: Vec<_> = list
        .windows(2)
        .map(|pair| format!("For want of a {} the {} was lost.\n", pair[0], pair[1]))
        .collect();

    format!("{}And all for the want of a {}.", result.join(""), list[0])
}
