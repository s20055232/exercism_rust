pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    let mut n = n;
    let mut three = false;
    let mut five = false;
    let mut seven = false;
    loop {
        if n % 3 == 0 && !three {
            result.push_str("Pling");
            n /= 3;
            three = true;
        } else if n % 5 == 0 && !five {
            result.push_str("Plang");
            n /= 5;
            five = true;
        } else if n % 7 == 0 && !seven {
            result.push_str("Plong");
            n /= 7;
            seven = true;
        } else {
            if result.is_empty() {
                result.push_str(&n.to_string());
            }
            break;
        }
    }
    result
}
