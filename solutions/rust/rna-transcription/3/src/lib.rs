#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    const VALID: [char; 4] = ['A', 'C', 'G', 'T'];
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(i) = dna.chars().position(|c| !Self::VALID.contains(&c)) {
            Err(i)
        } else {
            Ok(Self(dna.to_string()))
        }
    }

    pub fn into_rna(self) -> Rna {
        let rna = self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect();
        Rna(rna)
    }
}

impl Rna {
    const VALID: [char; 4] = ['A', 'C', 'G', 'U'];
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(i) = rna.chars().position(|c| !Self::VALID.contains(&c)) {
            Err(i)
        } else {
            Ok(Self(rna.to_string()))
        }
    }
}
