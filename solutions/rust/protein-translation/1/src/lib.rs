pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mapping = [
        ("AUG", Some("Methionine")),
        ("UUU", Some("Phenylalanine")),
        ("UUC", Some("Phenylalanine")),
        ("UUA", Some("Leucine")),
        ("UUG", Some("Leucine")),
        ("UCU", Some("Serine")),
        ("UCC", Some("Serine")),
        ("UCA", Some("Serine")),
        ("UCG", Some("Serine")),
        ("UAU", Some("Tyrosine")),
        ("UAC", Some("Tyrosine")),
        ("UGU", Some("Cysteine")),
        ("UGC", Some("Cysteine")),
        ("UGG", Some("Tryptophan")),
        ("UAA", None::<&str>),
        ("UAG", None::<&str>),
        ("UGA", None::<&str>),
    ];

    let mut codons: Vec<&str> = vec![];
    let mut iters: [Box<dyn Iterator<Item = char>>; 3] = [
        Box::new(rna.chars().step_by(3)),
        Box::new(rna.chars().skip(1).step_by(3)),
        Box::new(rna.chars().skip(2).step_by(3)),
    ];

    loop {
        let chars: Vec<Option<char>> = iters.iter_mut().map(|iter| iter.next()).collect();
        if chars.iter().all(|c| c.is_none()) {
            break;
        }
        if chars.iter().any(|c| c.is_none()) {
            return None;
        }
        let seq: String = chars.iter().map(|c| c.unwrap()).collect();
        let found = mapping.iter().find(|t| t.0 == seq);
        let codon = if let Some(t) = found {
            t.1
        } else {
            return None;
        };
        if let Some(codon) = codon {
            codons.push(codon);
        } else {
            break;
        }
    }

    Some(codons)
}
