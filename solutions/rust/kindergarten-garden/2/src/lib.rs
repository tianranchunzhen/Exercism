const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let pos = STUDENTS.iter().position(|&x| x == student).unwrap();

    diagram
        .split("\n")
        .flat_map(|line| line.chars().skip(pos * 2).take(2))
        .map(|s| match s {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => "",
        })
        .collect()
}