use std::collections::HashMap;

pub struct School(HashMap<u32, Vec<String>>);

impl School {
    pub fn new() -> School {
        Self(HashMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        match self
            .0
            .values()
            .any(|names| names.contains(&student.to_string()))
        {
            true => (),
            false => self.0.entry(grade).or_default().push(student.to_string()),
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.0.keys().copied().collect();
        grades.sort_unstable();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.0.get(&grade) {
            Some(names) => {
                let mut sort_names = names.clone();
                sort_names.sort();
                sort_names
            }
            None => Vec::new(),
        }
    }
}
