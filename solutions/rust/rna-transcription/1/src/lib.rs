#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (idx, char) in dna.chars().enumerate() {
            if !&['A', 'C', 'G', 'T'].contains(&char) {
                return Err(idx);
            }
        }

        Ok(Self(dna.into()))
    }

    pub fn into_rna(self) -> Rna {
        let mut rna = String::new();
        for char in self.0.chars() {
            let converted = match char {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => char,
            };
            rna.push(converted);
        }

        Rna(rna)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (idx, char) in rna.chars().enumerate() {
            if !&['A', 'C', 'G', 'U'].contains(&char) {
                return Err(idx);
            }
        }

        Ok(Self(rna.into()))
    }
}
