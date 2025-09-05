use std::collections::HashMap;

use itertools::Itertools;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide != 'A' && nucleotide != 'C' && nucleotide != 'G' && nucleotide != 'T' {
        Err(nucleotide)
    } else {
        let hashmap = nucleotide_counts(dna)?;
        Ok(hashmap.get(&nucleotide).cloned().unwrap_or(0))
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hashmap = dna.chars().counts();
    hashmap.entry('A').or_insert(0);
    hashmap.entry('C').or_insert(0);
    hashmap.entry('G').or_insert(0);
    hashmap.entry('T').or_insert(0);
    if let Some(&c) = hashmap
        .keys()
        .find(|&&key| key != 'A' && key != 'C' && key != 'G' && key != 'T')
    {
        Err(c)
    } else {
        Ok(hashmap)
    }
}
