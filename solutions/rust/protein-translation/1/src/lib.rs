use itertools::Itertools;

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut result: Vec<&str> = Vec::new();
    for chunk in &rna.chars().chunks(3) {
        let codon: String = chunk.collect();
        match codon.as_str() {
            "AUG" => {
                result.push("Methionine");
            }
            "UUU" | "UUC" => {
                result.push("Phenylalanine");
            }
            "UUA" | "UUG" => {
                result.push("Leucine");
            }
            "UCU" | "UCC" | "UCA" | "UCG" => {
                result.push("Serine");
            }
            "UAU" | "UAC" => {
                result.push("Tyrosine");
            }
            "UGU" | "UGC" => {
                result.push("Cysteine");
            }
            "UGG" => {
                result.push("Tryptophan");
            }
            "UAA" | "UAG" | "UGA" => {
                break;
            }
            _ => return None,
        }
    }
    Some(result)
}