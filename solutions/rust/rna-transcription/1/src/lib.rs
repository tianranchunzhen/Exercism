#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.chars().enumerate() {
            if c != 'A' && c != 'C' && c != 'G' && c != 'T' {
                return Err(i);
            }
        }
        Ok(Self {
            dna: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna = self
            .dna
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect();
        Rna { rna }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            if c != 'A' && c != 'C' && c != 'G' && c != 'U' {
                return Err(i);
            }
        }
        Ok(Self {
            rna: rna.to_string(),
        })
    }
}
