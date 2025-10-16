use anyhow::Error;
use itertools::Itertools;

#[derive(Debug)]
pub struct Flags {
    pub line_number: bool,      // -n
    pub list_files: bool,       // -l
    pub case_insensitive: bool, // -i
    pub invert: bool,           // -v
    pub entire_line: bool,      // -x
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            line_number: flags.contains(&"-n"),
            list_files: flags.contains(&"-l"),
            case_insensitive: flags.contains(&"-i"),
            invert: flags.contains(&"-v"),
            entire_line: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let pattern = if flags.case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    if flags.list_files {
        let mut matched_files = Vec::new();
        for file in files {
            if std::fs::read_to_string(file)?.lines().any(|line| {
                match (flags.entire_line, flags.case_insensitive, flags.invert) {
                    (true, true, false) => line.to_lowercase() == pattern,
                    (true, true, true) => line.to_lowercase() != pattern,
                    (true, false, false) => line == &pattern,
                    (true, false, true) => line != &pattern,
                    (false, true, false) => line.to_lowercase().contains(&pattern),
                    (false, true, true) => !line.to_lowercase().contains(&pattern),
                    (false, false, false) => line.contains(&pattern),
                    (false, false, true) => !line.contains(&pattern),
                }
            }) {
                matched_files.push(file.to_string());
            }
        }
        return Ok(matched_files);
    }
    
    let mut results = Vec::new();
    for file in files {
        std::fs::read_to_string(file)?
            .lines()
            .enumerate()
            .filter(
                |(_, line)| match (flags.entire_line, flags.case_insensitive, flags.invert) {
                    (true, true, false) => line.to_lowercase() == pattern,
                    (true, true, true) => line.to_lowercase() != pattern,
                    (true, false, false) => line == &pattern,
                    (true, false, true) => line != &pattern,
                    (false, true, false) => line.to_lowercase().contains(&pattern),
                    (false, true, true) => !line.to_lowercase().contains(&pattern),
                    (false, false, false) => line.contains(&pattern),
                    (false, false, true) => !line.contains(&pattern),
                },
            )
            .for_each(|(i, line)| results.push((file, i + 1, line.to_string())));
    }

    if flags.list_files {
        Ok(results
            .iter()
            .map(|(file, ..)| file.to_string())
            .unique()
            .collect())
    } else {
        Ok(results
            .iter()
            .map(
                |(file, i, line)| match (files.len() > 1, flags.line_number) {
                    (true, true) => format!("{file}:{i}:{line}"),
                    (true, false) => format!("{file}:{line}"),
                    (false, true) => format!("{i}:{line}"),
                    (false, false) => line.to_string(),
                },
            )
            .collect())
    }
}
