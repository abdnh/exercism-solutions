use std::collections::HashMap;

fn is_nucleotide(c: char) -> bool {
    matches!(c, 'A' | 'C' | 'G' | 'T')
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) {
        return Err(nucleotide);
    }
    let mut count = 0;
    for char in dna.chars() {
        if !is_nucleotide(char) {
            return Err(char);
        } else if char == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for char in dna.chars() {
        if !is_nucleotide(char) {
            return Err(char);
        } else {
            counts.entry(char).and_modify(|c| *c += 1).or_insert(1);
        }
    }

    Ok(counts)
}
