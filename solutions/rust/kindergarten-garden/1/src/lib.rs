const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn get_plant_name(s: &str) -> &'static str {
    match s {
        "G" => "grass",
        "C" => "clover",
        "R" => "radishes",
        "V" => "violets",
        _ => "",
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let pos = STUDENTS.iter().position(|&x| x == student).unwrap();
    diagram
        .split("\n")
        .map(|line| line.chars().skip(pos * 2).take(2).collect::<String>())
        .collect::<String>()
        .chars()
        .map(|s| get_plant_name(&s.to_string()))
        .collect()
}