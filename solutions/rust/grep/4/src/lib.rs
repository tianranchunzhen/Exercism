use anyhow::Error;

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

fn get_case_string(str: &str, flags: &Flags) -> String {
    if flags.case_insensitive {
        str.to_lowercase()
    } else {
        str.to_string()
    }
}

fn line_matches(line: &str, pattern: &str, flags: &Flags) -> bool {
    let line_to_compare = get_case_string(line, flags);
    if flags.entire_line {
        (&line_to_compare == pattern) ^ flags.invert
    } else {
        line_to_compare.contains(pattern) ^ flags.invert
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let pattern = get_case_string(pattern, flags);

    if flags.list_files {
        let mut matched_files = Vec::new();
        for file in files {
            if std::fs::read_to_string(file)?
                .lines()
                .any(|line| line_matches(line, &pattern, flags))
            {
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
            .filter(|(_, line)| line_matches(line, &pattern, flags))
            .for_each(|(i, line)| results.push((file, i + 1, line.to_string())));
    }

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
