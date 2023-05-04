use std::collections::HashMap;

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let student_place: HashMap<&str, (usize, usize)> = HashMap::from([
        ("Alice", (0, 1)),
        ("Bob", (2, 3)),
        ("Charlie", (4, 5)),
        ("David", (6, 7)),
        ("Eve", (8, 9)),
        ("Fred", (10, 11)),
        ("Ginny", (12, 13)),
        ("Harriet", (14, 15)),
        ("Ileana", (16, 17)),
        ("Joseph", (18, 19)),
        ("Kincaid", (20, 21)),
        ("Larry", (22, 23)),
    ]);
    let planets: Vec<String> = _diagram.lines().map(|s| s.to_owned()).collect();
    let mut result: Vec<&'static str> = Vec::new();
    let index = student_place
        .get(_student)
        .unwrap_or_else(|| panic!("Who are you? {_student}"))
        .to_owned();
    let planets = planets[0][(index.0)..=(index.1)].to_owned() + &planets[1][(index.0)..=(index.1)];
    for p in planets.chars() {
        match p {
            'V' => result.push("violets"),
            'G' => result.push("grass"),
            'R' => result.push("radishes"),
            'C' => result.push("clover"),
            _ => (),
        }
    }
    result
}
