const CONVERSION_MAP: [(&str, char); 10] = [
    (" _ | ||_|   ", '0'),
    ("     |  |   ", '1'),
    (" _  _||_    ", '2'),
    (" _  _| _|   ", '3'),
    ("   |_|  |   ", '4'),
    (" _ |_  _|   ", '5'),
    (" _ |_ |_|   ", '6'),
    (" _   |  |   ", '7'),
    (" _ |_||_|   ", '8'),
    (" _ |_| _|   ", '9'),
];

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let rows: Vec<&str> = input.lines().collect();
    if !rows.len().is_multiple_of(4) {
        return Err(Error::InvalidRowCount(rows.len()));
    }
    for len in rows.iter().map(|row| row.len()) {
        if !len.is_multiple_of(3) {
            return Err(Error::InvalidColumnCount(len));
        }
    }

    let res = rows
        .chunks(4)
        .map(|chunk| {
            (0..chunk[0].len())
                .step_by(3)
                .map(|idx| {
                    let num_str: String = chunk
                        .iter()
                        .flat_map(|row| row[idx..idx + 3].chars())
                        .collect();
                    CONVERSION_MAP
                        .iter()
                        .find(|(str, _)| str == &num_str)
                        .map_or('?', |(_, num)| *num)
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();
    Ok(res.join(","))
}
