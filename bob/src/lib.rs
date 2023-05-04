pub fn reply(message: &str) -> &str {
    let question = message.trim().ends_with('?');
    let all_upper = message
        .chars()
        .filter(|x| x.is_alphabetic())
        .all(|x| x.is_uppercase());
    let have_character = message
        .chars()
        .filter(|x| x.is_alphabetic())
        .collect::<Vec<_>>()
        .len()
        > 1;
    let empty = message.trim().is_empty();
    // 如果句尾為“？”，有字母（abc）不為空字串且全部都是大寫
    if question && all_upper && !empty && have_character {
        "Calm down, I know what I'm doing!"
    // 如果句尾是“？”
    } else if question {
        "Sure."
    // 有字母（abc）不為空字串且全部都是大寫
    } else if all_upper && !empty && have_character {
        "Whoa, chill out!"
    // 不為空字串
    } else if empty {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
    // unimplemented!("have Bob reply to the incoming message: {message}")
}
